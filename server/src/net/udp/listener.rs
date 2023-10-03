use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::sync::Notify;

use binary_util::{
    interfaces::{Reader, Writer},
    io::ByteReader,
};

use super::conn::Conn;
use crate::utils::current_epoch;
use crate::utils::PossiblySocketAddr;
use protocol::net::udp::proto::offline::{Ping, Pong};
use protocol::net::udp::proto::{offline::OfflinePackets, Packets};

pub(crate) type ConnMap = Arc<Mutex<HashMap<SocketAddr, Conn>>>;

#[derive(Debug, PartialEq)]
pub enum ListenerState {
    Ready,
    Running,
    Closed,
}

pub struct Listener {
    pub addr: SocketAddr,
    /// This is a notifier to kill the listener and all of it's tasks.
    /// When this is notified, the listener will close.
    close_notifier: Arc<Notify>,
    socket: Arc<tokio::net::UdpSocket>,
    /// A mpsc channel that will send connections to the user and back to the listener.
    rx_accept_channel: Receiver<Conn>,
    tx_accept_channel: Sender<Conn>,
    /// This is a hash_map of all connections, it contains a buffer channel
    /// that will send data to the connection.
    connections: ConnMap,
    state: ListenerState,
}

impl Listener {
    pub async fn bind<I: for<'a> Into<PossiblySocketAddr<'a>>>(
        address: I,
    ) -> Result<Self, std::io::Error> {
        let addr = (address.into() as PossiblySocketAddr).to_socket_addr();
        let close_notifier = Arc::new(Notify::new());

        let (tx_accept_channel, rx_accept_channel) = tokio::sync::mpsc::channel::<Conn>(5);
        let connections = Arc::new(Mutex::new(HashMap::<SocketAddr, Conn>::new()));

        if let None = addr {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Bind address is invalid.",
            ));
        }

        let socket = match tokio::net::UdpSocket::bind(addr.unwrap()).await {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, e));
            }
        };

        let socket = Arc::new(socket);

        Ok(Self {
            addr: addr.unwrap(),
            close_notifier,
            rx_accept_channel,
            tx_accept_channel,
            connections,
            socket,
            state: ListenerState::Ready,
        })
    }

    pub async fn start(&mut self) -> std::io::Result<()> {
        if self.state != ListenerState::Ready {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Listener is already running.",
            ));
        }

        self.state = ListenerState::Running;

        let mut buf = [0u8; 1024];
        let socket = self.socket.clone();
        let notifier = self.close_notifier.clone();
        let connections = self.connections.clone();

        tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    _ = notifier.notified() => {
                        break;
                    }
                    res = socket.recv_from(&mut buf) => {
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

                        let mut reader = ByteReader::from(&buf[..len]);

                        if let Ok(packet) = OfflinePackets::read(&mut reader) {
                            match packet {
                                OfflinePackets::Ping(ping) => {
                                    let pong = Pong {
                                        send: ping.send,
                                        recv: current_epoch()
                                    };

                                    send_packet_to(&socket, addr, Packets::Pong(pong)).await.unwrap();
                                }
                                OfflinePackets::ConnectRequest(request) => {
                                    // todo: Check if ip is banned.
                                    // AFTER THIS PACKET IS SENT, THE CONNECTION
                                    // IS HANDLED ENTIRELY BY THE CONN STRUCT.
                                    let mut sessions = connections.lock().await;

                                    if !sessions.contains_key(&addr) {
                                        // let conn = Conn::new(addr, socket.clone(), connections.clone());
                                        // sessions.insert(addr, conn);
                                    }

                                    todo!()
                                },
                                _ => {}
                            };
                        }
                    }
                }
            }
        });

        todo!()
    }

    pub async fn close(&self) {
        self.close_notifier.notify_waiters();
    }
}

async fn send_packet_to(
    socket: &Arc<UdpSocket>,
    to: SocketAddr,
    packet: Packets,
) -> std::io::Result<()> {
    if let Ok(b) = packet.write_to_bytes() {
        socket.send_to(&b.as_slice(), to).await?;
    }

    Ok(())
}
