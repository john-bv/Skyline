use std::collections::HashMap;
use std::time::SystemTime;
use std::{net::SocketAddr, sync::Arc};
use std::io::{self, BufReader, BufRead};

use async_trait::async_trait;
use binary_util::ByteReader;
use binary_util::interfaces::Writer;
use protocol::net::tcp::{SplitPacket, Messages, HeartbeatAck, Payload};
use protocol::skyline::{connection::{DisconnectReason, Disconnect}, SkylinePacket};
use tokio::io::AsyncReadExt;
use tokio::{sync::{Notify, RwLock}, io::AsyncWriteExt};

use crate::net::{ConnState, ConnAdapter};

pub struct Conn {
    pub addr: SocketAddr,
    pub state: ConnState,
    close_notifier: Arc<Notify>,
    /// Single channel for digesting skyline packets.
    net_rx: tokio::sync::mpsc::Receiver<SkylinePacket>,

    socket: Arc<tokio::net::TcpStream>,
    /// This is a queue of sent packets that have been split.
    splits: RwLock<HashMap<u16, (SystemTime, Vec<SplitPacket>)>>,
}

impl Conn {
    pub fn new(mut stream: tokio::net::TcpStream) -> Self {
        // initialize new notifier;
        let close_notifier = Arc::new(Notify::new());
        let addr = stream.peer_addr().unwrap();

        // move a dedicated task to handle the stream.
        let (net_tx, mut net_rx) = tokio::sync::mpsc::channel::<&[u8]>(100);
        let (pak_tx, mut pak_rx) = tokio::sync::mpsc::channel::<SkylinePacket>(100);

        let mut socket = Arc::new(stream);
        let self_socket = Arc::clone(&socket);

        let heartbeat_socket = Arc::clone(&socket);
        let heartbeat_task = tokio::task::spawn(async move {
            loop {
                if let Ok(_) = heartbeat_socket.writable().await {
                    let heartbeat = Messages::HeartbeatAck(HeartbeatAck {
                        timestamp: SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });

                    if let Err(_) = Self::send_packet(&heartbeat_socket, &mut HashMap::new(), heartbeat).await
                    {
                        println!("[{}] Error: Failed to send heartbeat packet...", addr);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });

        let net_closer = Arc::clone(&close_notifier);
        tokio::spawn(async move {
            // loop until closed or disconnect
            let mut buf: [u8; 1024] = [0; 1024];
            // todo: this is hacky but works for now.
            //       currently we use custom TCP proto (while small) not fully implemented.
            let mut current = Vec::new();
            let mut err: Option<std::io::Error> = None;

            loop {
                tokio::select! {
                    _ = net_closer.notified() => {
                        break;
                    }
                    _ = socket.readable() => {
                        // todo: Modify binary-util to allow for a stream to be passed in.
                        //       this will make it easier to read packets from a stream.
                        match socket.try_read(&mut buf) {
                            Ok(0) => {
                                // No data was read...
                                break;
                            }
                            Ok(n) => {
                                // read n bytes
                                current.extend(&buf[..n]);

                                let mut reader = ByteReader::from(&current[..]);
                                reader.read_type::<SkylinePacket>().unwrap();
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                                println!("[{}] Client disconnected", addr);
                                break;
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::ConnectionAborted => {
                                println!("[{}] Client disconnected", addr);
                                break;
                            }
                            Err(e) => {
                                err = Some(e);
                                break;
                            }
                        }
                    }
                }
            }

            // notify the closer
            net_closer.notify_waiters();
        });

        Self {
            addr: addr,
            state: ConnState::Connecting,
            close_notifier,
            net_rx: pak_rx,
            socket: self_socket,
            splits: RwLock::new(HashMap::new()),
        }
    }

    async fn send_packet(socket: &Arc<tokio::net::TcpStream>, splits: &mut HashMap<u16, (SystemTime, Vec<SplitPacket>)>, packet: Messages) -> std::io::Result<()> {
        let packet = packet.write_to_bytes().unwrap();
        let buf = packet.as_slice();

        if buf.len() >= (1024 - 60 - 12 - 100) {
            // remove old splits
            for (id, (time, _)) in splits.clone() {
                if time.elapsed().unwrap().as_secs() > 10 {
                    splits.remove(&id);
                }
            }

            // get next available id
            let next = splits.len() + 1;
            let split_pks = SplitPacket::split(next as u16, buf).unwrap();
            splits.insert(next as u16, (SystemTime::now(), split_pks.clone()));

            for split_pk in split_pks {
                let x = Messages::SplitPacket(split_pk)
                    .write_to_bytes()
                    .unwrap();
                let bin = x.as_slice();

                if let Err(_) = socket.writable().await {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Write Error"));
                }

                let frame = protocol::net::tcp::Frame::new(bin.to_vec()).write_to_bytes().unwrap();
                if let Err(_) = socket.try_write(&frame.as_slice()) {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Write Error"));
                }
            }

            return Ok(());
        } else {
            let frame = protocol::net::tcp::Frame::new(buf.to_vec()).write_to_bytes().unwrap();
            if let Err(_) = socket.try_write(&frame.as_slice()) {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Write Error"));
            } else {
                return Ok(());
            }
        }
    }
}

#[async_trait]
impl ConnAdapter for Conn {
    async fn close(&self, reason: DisconnectReason) -> std::io::Result<()> {
        self.close_notifier.notify_waiters();
        let disconnect = Disconnect {
            reason: DisconnectReason::InvalidProtocol
        };

        self.send(&SkylinePacket::Disconnect(disconnect)).await?;
        self.send_message(Messages::Disconnect(protocol::net::tcp::Disconnect::SelfInitiated)).await?;
        Ok(())
    }

    async fn send(&self, packet: &SkylinePacket) -> std::io::Result<()> {
        // write this buffer
        let x = packet.write_to_bytes().unwrap().as_slice().to_vec();

        let tcp_pk = Messages::Payload(Payload {
            data: x,
        });

        let mut splits = self.splits.write().await;

        if let Err(_) = Self::send_packet(&self.socket, &mut splits, tcp_pk).await {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Write Error"));
        }

        return Ok(());
    }

    async fn send_message(&self, message: protocol::net::tcp::Messages) -> std::io::Result<()> {
        // this will internally attempt to send the tcp packet.
        let mut splits = self.splits.write().await;

        if let Err(_) = Self::send_packet(&self.socket, &mut splits, message).await {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Write Error"));
        }

        return Ok(());
    }

    async fn recv(&mut self) -> Result<protocol::skyline::SkylinePacket, std::io::Error> {
        let packet = self.net_rx.recv().await;
        if let Some(packet) = packet {
            return Ok(packet);
        }

        return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Channel closed"));
    }
}