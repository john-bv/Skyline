use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use binary_util::interfaces::Writer;

use super::{
    recovery::RecoveryQueue,
    split::{SplitQueue, SplitQueueError},
};
use crate::{
    net::udp::proto::{
        online::{
            ack::Acknowledgeable,
            dataset::{DataBit, DataSet, Datagram, OrderInfo, SplitInfo},
            OnlinePackets,
        },
        Packets, MAX_PROTO_OVERHEAD,
    },
    net::udp::queue::{ord, NetQueue},
    util::SafeGenerator,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SendPriority {
    /// This packet is not important, it can be dropped if needed.
    /// Same as `Unreliable`, it will not be resent.
    /// These packets are sent immediately, but will be dropped if the queue is full.
    Low,
    /// This packet is important, and needs a reply but we don't need to
    /// send it immediately.
    Medium,
    /// This packet is critical, it should be pushed to the front of the queue
    High,
    /// This packet is critical, it should be sent immediately.
    Immediate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SendQueueError {
    /// The packet is too large to be sent, try sending a smaller packet.
    PacketTooLarge,
    /// There was an error when splitting the packet.
    SplitError(SplitQueueError),
    /// Could not send the packet.
    SendError,
}

pub struct SendQueue {
    mtu_size: u16,
    /// The current sequence number.
    /// This is incremented every time a packet is sent.
    seq: SafeGenerator<u32>,
    /// The reliable sequence number
    reliable_seq: SafeGenerator<u32>,
    ack: RecoveryQueue<Datagram>,
    splitq: SplitQueue,
    /// channels
    /// (channel, seq)
    ord_chans: HashMap<u16, (u32, u32)>,
    /// The packets we're ready to process
    queue: HashMap<SendPriority, Vec<DataSet>>,
    socket: Arc<tokio::net::UdpSocket>,
    address: SocketAddr,
}

impl SendQueue {
    pub fn new(
        mtu: u16,
        socket: Arc<tokio::net::UdpSocket>,
        address: std::net::SocketAddr,
    ) -> Self {
        Self {
            mtu_size: mtu,
            seq: SafeGenerator::new(),
            ack: RecoveryQueue::new(),
            splitq: SplitQueue::new(),
            ord_chans: HashMap::new(),
            queue: HashMap::new(),
            socket,
            address,
            reliable_seq: SafeGenerator::new(),
        }
    }

    pub async fn insert_writable(
        &mut self,
        any_writer: impl Writer,
        priority: SendPriority,
        channel: Option<u16>,
    ) -> Result<(), SendQueueError> {
        let bytes = match any_writer.write_to_bytes() {
            Ok(b) => b,
            Err(_) => {
                return Err(SendQueueError::SendError);
            }
        };

        self.insert(bytes.as_slice(), priority, channel).await
    }

    pub async fn insert(
        &mut self,
        packet: &[u8],
        mut priority: SendPriority,
        channel: Option<u16>,
    ) -> Result<(), SendQueueError> {
        // we will modify bits depending on the payload size...
        // if a packet is split, it will be marked as reliable.
        if packet.len() > (self.mtu_size + MAX_PROTO_OVERHEAD) as usize {
            priority = SendPriority::Medium;
        }

        // check the priority
        match priority {
            SendPriority::Low => {
                // we don't care about this packet, we can drop it if needed.
                self.send_set(
                    DataSet::new()
                        .with_payload(packet.to_vec())
                        .with_bits(DataBit::new().with_unreliable()),
                )
                .await?;
            }
            _ => {}
        };

        if packet.len() > (self.mtu_size + MAX_PROTO_OVERHEAD) as usize {
            // we need to split this packet
            let mut datagram = Datagram::new().with_sequence(self.seq.next().into());

            let split_insert = self.splitq.split_insert(&packet, self.mtu_size);
            if let Ok(split_id) = split_insert {
                let (_, parts) = self.splitq.get_mut(&split_id).unwrap();
                let (ord_seq, ord_idx) =
                    self.ord_chans.entry(channel.unwrap_or(0)).or_insert((0, 0));

                for part in parts.iter_mut() {
                    part.flags = DataBit::new().with_split().with_reliable().with_ordered();

                    part.seq = self.seq.next().into();
                    part.reliable_seq = Some(self.reliable_seq.next().into());
                    part.order = Some(OrderInfo {
                        id: channel.unwrap_or(0),
                        index: *ord_idx,
                        sequence: *ord_seq,
                    });

                    datagram.push_set(part.clone());
                }

                *ord_idx += 1;
                *ord_seq += 1;

                // now we need to add all packets in this sequence to the ack queue.
                // we're doing this in case the packet is lost, we can resend it.
                if let Err(_) = self.ack.insert(datagram) {
                    println!("failed to insert ack packet");
                }

                // we have queued the packets, will be sent next tick.
                return Ok(());
            }

            return Err(SendQueueError::SplitError(split_insert.unwrap_err()));
        } else {
            // this packet isn't split, we can send it immediately (if defined by the priority)
            let mut set = DataSet::new().with_payload(packet.to_vec());

            match priority {
                SendPriority::High => {
                    set.flags = set.flags.with_reliable();
                    set.reliable_seq = Some(self.reliable_seq.next().into());
                    self.queue
                        .entry(SendPriority::High)
                        .or_insert(Vec::new())
                        .push(set);
                    Ok(())
                }
                SendPriority::Medium => {
                    set.flags = set.flags.with_reliable();
                    set.reliable_seq = Some(self.reliable_seq.next().into());
                    self.queue
                        .entry(SendPriority::Medium)
                        .or_insert(Vec::new())
                        .push(set);
                    Ok(())
                }
                SendPriority::Immediate => {
                    self.send_set(set).await?;
                    Ok(())
                }
                _ => Err(SendQueueError::PacketTooLarge),
            }
        }
    }

    /// A wrapper to send a single set over the network.
    /// This will automatically set the sequence number and reliable sequence number.
    async fn send_set(&mut self, mut set: DataSet) -> Result<(), SendQueueError> {
        // we need to verify that the packet is proper for the payload.

        if set.flags.is_reliable() {
            set.reliable_seq = Some(self.reliable_seq.next().into());
        }

        let datagram = Datagram::new()
            .with_sequence(self.seq.next().into())
            .with_set(set);

        let packet = Packets::OnlinePacket(OnlinePackets::Datagram(datagram));

        if let Ok(buf) = packet.write_to_bytes() {
            return self.send_raw(buf.as_slice()).await;
        }

        Ok(())
    }

    async fn send_datagram(&mut self, datagram: Datagram) -> Result<(), SendQueueError> {
        let datagram = Packets::OnlinePacket(OnlinePackets::Datagram(datagram));
        if let Ok(buf) = datagram.write_to_bytes() {
            return self.send_raw(buf.as_slice()).await;
        } else {
            return Err(SendQueueError::SendError);
        }
    }

    pub async fn send_raw(&mut self, packet: &[u8]) -> Result<(), SendQueueError> {
        if let Err(e) = self.socket.send_to(packet, &self.address).await {
            return Err(SendQueueError::SendError);
        } else {
            return Ok(());
        }
    }

    pub async fn update(&mut self) {
        // high priority packets sent FIRST
        for pk in self
            .queue
            .get_mut(&SendPriority::High)
            .unwrap()
            .drain(..)
            .collect::<Vec<DataSet>>()
        {
            if let Err(_) = self.send_set(pk).await {
                println!("failed to send high priority packet");
            }
        }
        // medium priority packets sent SECOND
        for pk in self
            .queue
            .get_mut(&SendPriority::Medium)
            .unwrap()
            .drain(..)
            .collect::<Vec<DataSet>>()
        {
            if let Err(_) = self.send_set(pk).await {
                println!("failed to send medium priority packet");
            }
        }

        // clear old ACKs
        for pk in self.ack.flush_old(1000).drain(..) {
            if let Err(_) = self.send_datagram(pk).await {
                println!("failed to send ack packet");
            }
        }
    }
}

impl Acknowledgeable for SendQueue {
    type NackItem = Datagram;

    fn ack(&mut self, ack: crate::net::udp::proto::online::ack::Acknowledgement) {
        // we are ackowledging a packet!
        // remove each item from the queue
        ack.seqs.into_iter().for_each(|seq| {
            if let Err(_) = self.ack.remove(seq) {};
        });
    }

    fn nack(
        &mut self,
        nack: crate::net::udp::proto::online::ack::Acknowledgement,
    ) -> Vec<Self::NackItem> {
        nack.seqs
            .into_iter()
            .filter_map(|seq| {
                if let Ok(v) = self.ack.get(seq) {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
