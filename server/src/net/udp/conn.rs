use std::{
    net::{SocketAddr, UdpSocket},
    sync::{atomic::AtomicU64, Arc},
};

use binary_util::{
    interfaces::{Reader, Writer},
    ByteReader,
};
use protocol::{
    net::udp::proto::{
        online::{
            ack::{AckVariant, Acknowledgeable, Acknowledgement},
            OnlinePackets,
        },
        Packets,
    },
    net::udp::queue::{
        recv::RecvQueue,
        send::{SendPriority, SendQueue},
    },
};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, Notify, RwLock,
    },
    task::JoinHandle,
};

pub(crate) type ConnNetChan = Arc<Mutex<Receiver<Vec<u8>>>>;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ConnState {
    Offline,
    Connecting,
    Connected,
    TimingOut,
    Disconnected,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ProcessingStatus {
    Disconnect,
    NotAllowed,
    Ok,
}

/// Primary structure for handling concurrent connections with
/// the skyline protocol.
///
/// This does not implement the advanced features of the protocol,
/// but is rather a barebones implementation of the protocol.
pub struct Conn {
    addr: SocketAddr,
    state: Mutex<ConnState>,
    socket: Arc<tokio::net::UdpSocket>,
    /// The api for Conn::recv()
    network_recv: ConnNetChan,
    disconnect: Arc<Notify>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
    send_q: Arc<RwLock<SendQueue>>,
    recv_q: Arc<Mutex<RecvQueue>>,
    last_recv: Arc<AtomicU64>,
}

impl Conn {
    pub async fn new(
        addr: SocketAddr,
        socket: &Arc<tokio::net::UdpSocket>,
        cleanup: Arc<Sender<SocketAddr>>,
        mtu: u16,
    ) -> Self {
    }

    /// This thread will handle the connection tick.
    /// The tick is responsible for timing out the connection,
    /// if packets are not recieved within a certain time frame.
    pub async fn init_tick(&self, cleanup: Arc<Sender<SocketAddr>>) -> JoinHandle<()> {
        let notifier = self.disconnect.clone();

        tokio::task::spawn(async move {

            //
        })
    }

    /// Handles the incoming messages and processes them, the only messages allowed
    /// are connection packets.
    /// Any other packets will be dropped or ignored, this is to prevent any
    /// malicious packets from being sent to the server.
    pub async fn init_network(
        &self,
        cleanup: Arc<Sender<SocketAddr>>,
        sender: Sender<Vec<u8>>,
    ) -> JoinHandle<()> {
        let notifier = self.disconnect.clone();
        let socket = self.socket.clone();
        let address = self.addr.clone();
        let send_q = self.send_q.clone();
        let recv_q = self.recv_q.clone();
        let last_recv = self.last_recv.clone();

        tokio::task::spawn(async move {
            let mut buf: [u8; 1024] = [0u8; 1024];
            'recv: loop {
                tokio::select! {
                    _ = notifier.notified() => {
                        break;
                    },
                    res = socket.recv_from(&mut buf) => {
                        // process the packet
                        // if it's a connection packet, process it.
                        // if it's not, drop it.
                        let len: usize;
                        let addr: SocketAddr;
                        match res {
                            Ok((l, o)) => {
                                len = l;
                                addr = o;
                            },
                            Err(e) => {
                                continue;
                            }
                        }

                        // todo: It might be better to use a mpsc channel
                        //       instead here...
                        if addr != address {
                            continue;
                        }

                        last_recv.store(protocol::util::current_epoch(), std::sync::atomic::Ordering::Release);

                        let mut reader = ByteReader::from(&buf[..len]);

                        if let Ok(packet) = Packets::read(&mut reader) {
                            let packet = match packet {
                                Packets::OnlinePacket(v) => v,
                                _ => {
                                    continue;
                                }
                            };

                            match packet {
                                OnlinePackets::Datagram(datagram) => {
                                    let mut recv_queue = recv_q.lock().await;

                                    if let Err(e) = recv_queue.insert(datagram) {
                                        println!("Failed to insert datagram into recv queue: {:?}", e);
                                    }

                                    // flush the queue
                                    let in_sent = recv_queue.flush();

                                    for pk in in_sent.iter() {
                                        let status = Self::process_packet(&pk, &addr, &sender, &send_q).await;
                                        if status == ProcessingStatus::Disconnect {
                                            notifier.notify_waiters();
                                            break 'recv;
                                        }
                                    }

                                    drop(recv_queue);
                                },
                                OnlinePackets::Ack(variant) => {
                                    match variant {
                                        AckVariant::Ack(ack) => Self::process_ack(ack, &send_q, &recv_q, true).await,
                                        AckVariant::Nack(nack) => Self::process_ack(nack, &send_q, &recv_q, false).await,
                                    };
                                },
                                _ => {
                                    // invalid packet
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        })
    }

    pub(crate) async fn process_ack(
        ack: Acknowledgement,
        send_q: &Arc<RwLock<SendQueue>>,
        recv_q: &Arc<Mutex<RecvQueue>>,
        is_ack: bool,
    ) -> () {
        let mut send_queue = send_q.write().await;
        let mut recv_queue = recv_q.lock().await;

        if is_ack {
            send_queue.ack(ack);
        } else {
            let resend = send_queue.nack(ack);
            for datagram in resend.iter() {
                // get each set and resend it.
                let payload = send_queue
                    .send_raw(datagram.write_to_bytes().unwrap().as_slice())
                    .await
                    .unwrap();
            }
        }
    }

    pub(crate) async fn process_packet(
        stream: &[u8],
        addr: &SocketAddr,
        conn_sender: &Sender<Vec<u8>>,
        send_q: &Arc<RwLock<SendQueue>>,
    ) -> ProcessingStatus {
        ProcessingStatus::Ok
    }
}
