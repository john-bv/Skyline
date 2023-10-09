use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;
use std::{net::SocketAddr, sync::Arc};

use async_recursion::async_recursion;
use async_trait::async_trait;
use binary_util::interfaces::{Reader, Writer};
use binary_util::ByteReader;
use protocol::net::tcp::{Disconnect, HeartbeatAck, Hello, Messages, Payload, SplitPacket};
use protocol::skyline::{connection::DisconnectReason, SkylinePacket};
use protocol::util::current_epoch;
use tokio::sync::{Notify, RwLock};

use crate::net::{ConnAdapter, ConnState};

pub struct Conn {
    pub addr: SocketAddr,
    pub state: ConnState,
    close_notifier: Arc<Notify>,
    /// Single channel for digesting skyline packets.
    net_rx: tokio::sync::mpsc::Receiver<SkylinePacket>,
    socket: Arc<tokio::net::TcpStream>,
    /// This is a queue of sent packets that have been split.
    splits: Arc<RwLock<HashMap<u16, (SystemTime, Vec<SplitPacket>)>>>,
    /// Tasks that are spawned by this connection.
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Conn {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        // initialize new notifier;
        let close_notifier = Arc::new(Notify::new());
        let addr = stream.peer_addr().unwrap();
        let (pak_tx, pak_rx) = tokio::sync::mpsc::channel::<SkylinePacket>(100);

        let socket = Arc::new(stream);
        let self_socket = Arc::clone(&socket);

        let heartbeat_socket = Arc::clone(&socket);
        let heartbeat_notifier = Arc::clone(&close_notifier);
        let heartbeat_task = tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    _ = heartbeat_notifier.notified() => {
                        break;
                    }
                    _ = heartbeat_socket.writable() => {
                        let heartbeat = Messages::HeartbeatAck(HeartbeatAck {
                            timestamp: SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });

                        if let Err(_) =
                            Self::send_packet(&heartbeat_socket, &mut HashMap::new(), heartbeat).await
                        {
                            println!("[{}] Error: Failed to send heartbeat packet...", addr);
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    }
                };
            }
        });

        let net_closer = Arc::clone(&close_notifier);
        let recv_splits = Arc::new(RwLock::new(HashMap::new()));

        let recv_splits_net = Arc::clone(&recv_splits);

        let recv_task = tokio::spawn(async move {
            // loop until closed or disconnect
            let mut buf: [u8; 1024] = [0; 1024];
            // todo: this is hacky but works for now.
            //       currently we use custom TCP proto (while small) not fully implemented.
            let mut current = Vec::new();
            let mut split_parts: BTreeMap<u16, BTreeMap<u16, SplitPacket>> = BTreeMap::new();

            'outer: loop {
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

                                // check the current buffer for a frame.
                                let mut reader = ByteReader::from(&current[..]);

                                'pk_lp: loop {
                                    if reader.as_slice().len() < 2 {
                                        break 'pk_lp;
                                    }

                                    if current[1] != 54 {
                                        println!("[{}] Invalid frame ID", addr);
                                        current.clear();
                                        break;
                                    }

                                    let mut recv_splits = recv_splits_net.write().await;

                                    match reader.read_type::<protocol::net::tcp::Frame>() {
                                        Ok(frame) => {
                                            if let Err(e) = Self::process_tcp_message(
                                                &socket,
                                                &frame.message,
                                                &mut split_parts,
                                                &mut recv_splits,
                                                &pak_tx
                                            ).await {
                                                println!("[{}] Error: {}", addr, e);
                                                break 'outer;
                                            }
                                        }

                                        Err(e) => {
                                            println!("[{}] Error: {}", addr, e);
                                            break 'pk_lp;
                                        }
                                    }
                                }
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
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            }

            // notify the closer
            net_closer.notify_waiters();
        });

        let mut tasks = Vec::new();
        tasks.push(heartbeat_task);
        tasks.push(recv_task);

        Self {
            addr: addr,
            state: ConnState::Connecting,
            close_notifier,
            net_rx: pak_rx,
            socket: self_socket,
            splits: recv_splits,
            tasks,
        }
    }

    #[async_recursion]
    async fn process_tcp_message(
        socket: &Arc<tokio::net::TcpStream>,
        buf: &[u8],
        recv_splits: &mut BTreeMap<u16, BTreeMap<u16, SplitPacket>>,
        send_splits: &mut HashMap<u16, (SystemTime, Vec<SplitPacket>)>,
        sender: &tokio::sync::mpsc::Sender<SkylinePacket>,
    ) -> std::io::Result<()> {
        let addr = socket.peer_addr().unwrap();
        if let Ok(message) = Messages::read_from_slice(&buf) {
            match message {
                Messages::Disconnect(reason) => {
                    println!("[{}] Client disconnected: {:?}", addr, reason);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client disconnected",
                    ));
                }

                Messages::Connect(connect) => {
                    // send a hello packet.
                    if connect.version > protocol::net::tcp::PROTOCOL_VERSION {
                        println!("[{}] Error: Client protocol version is too new", addr);
                        if let Err(_) = Self::send_disconnect(
                            &socket,
                            protocol::net::tcp::Disconnect::InvalidProtocol,
                        )
                        .await
                        {}
                        // recommend disconnect.
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Client protocol version is too new",
                        ));
                    }

                    if connect.version < protocol::net::tcp::PROTOCOL_VERSION {
                        println!("[{}] Error: Client protocol version is too old", addr);
                        if let Err(_) = Self::send_disconnect(
                            &socket,
                            protocol::net::tcp::Disconnect::InvalidProtocol,
                        )
                        .await
                        {}
                        // recommend disconnect.
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Client protocol version is too old",
                        ));
                    }

                    // send hello.
                    let hello = Hello {
                        interval: 10_u16,
                        timestamp: Some(current_epoch()),
                    };

                    let hello = Messages::Hello(hello);

                    if let Err(_) = Self::send_packet(socket, send_splits, hello).await {
                        // dont error here...
                    }
                }

                Messages::HeartbeatAck(heartbeat) => {
                    println!("[{}] Heartbeat: {}", addr, heartbeat.timestamp);
                }

                Messages::SplitOk(split_ok) => {
                    send_splits.remove(&split_ok.id);
                }

                Messages::SplitPacket(split) => {
                    if !recv_splits.contains_key(&split.id) {
                        recv_splits.insert(split.id, BTreeMap::new());
                    }

                    let parts = recv_splits.get_mut(&split.id).unwrap();

                    if !parts.contains_key(&split.index) {
                        parts.insert(split.index, split.clone());
                    }

                    if parts.len() as u16 == split.size {
                        let mut buffer = Vec::new();

                        for i in 0..split.size {
                            let part = parts.get(&i).unwrap();
                            buffer.extend(&part.data);
                        }

                        let split_ok =
                            Messages::SplitOk(protocol::net::tcp::SplitOk { id: split.id });

                        if let Err(_) = Self::send_packet(socket, send_splits, split_ok).await {
                            // dont error here...
                        }

                        recv_splits.remove(&split.id);

                        return Self::process_tcp_message(
                            socket,
                            &buffer,
                            recv_splits,
                            send_splits,
                            sender,
                        )
                        .await;
                    } else {
                        return Ok(());
                    }
                }
                Messages::Payload(payload) => {
                    if let Ok(packet) = SkylinePacket::read_from_slice(&payload.data) {
                        if let Ok(_) = sender.send(packet).await {};
                    } else {
                        println!("[{}] Error: Failed to read skyline packet", addr);
                    }
                }
                _ => {
                    println!("[{}] Error: Unknown message type", addr);
                }
            }

            return Ok(());
        } else {
            println!("[{}] Error: Failed to read message", addr);
        }

        return Ok(());
    }

    async fn send_disconnect(
        socket: &Arc<tokio::net::TcpStream>,
        reason: Disconnect,
    ) -> std::io::Result<()> {
        let disconnect = Messages::Disconnect(reason);

        if let Err(_) = Self::send_packet(socket, &mut HashMap::new(), disconnect).await {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Send Error"));
        } else {
            return Ok(());
        }
    }

    async fn send_packet(
        socket: &Arc<tokio::net::TcpStream>,
        splits: &mut HashMap<u16, (SystemTime, Vec<SplitPacket>)>,
        packet: Messages,
    ) -> std::io::Result<()> {
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
                let x = Messages::SplitPacket(split_pk).write_to_bytes().unwrap();
                let bin = x.as_slice();

                if let Err(_) = socket.writable().await {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Write Error",
                    ));
                }

                let frame = protocol::net::tcp::Frame::new(bin.to_vec())
                    .write_to_bytes()
                    .unwrap();
                if let Err(_) = socket.try_write(&frame.as_slice()) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Write Error",
                    ));
                }
            }

            return Ok(());
        } else {
            let frame = protocol::net::tcp::Frame::new(buf.to_vec())
                .write_to_bytes()
                .unwrap();
            if let Err(_) = socket.try_write(&frame.as_slice()) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Write Error",
                ));
            } else {
                return Ok(());
            }
        }
    }
}

#[async_trait]
impl ConnAdapter for Conn {
    async fn close(&mut self, reason: DisconnectReason) -> std::io::Result<()> {
        self.close_notifier.notify_waiters();
        let disconnect = protocol::skyline::connection::Disconnect { reason };

        self.send(&SkylinePacket::Disconnect(disconnect)).await?;
        self.send_message(Messages::Disconnect(
            protocol::net::tcp::Disconnect::SelfInitiated,
        ))
        .await?;

        for task in self.tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    async fn send(&self, packet: &SkylinePacket) -> std::io::Result<()> {
        // write this buffer
        let x = packet.write_to_bytes().unwrap().as_slice().to_vec();

        let tcp_pk = Messages::Payload(Payload { data: x });

        let mut splits = self.splits.write().await;

        if let Err(_) = Self::send_packet(&self.socket, &mut splits, tcp_pk).await {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Write Error",
            ));
        }

        return Ok(());
    }

    async fn send_message(&self, message: protocol::net::tcp::Messages) -> std::io::Result<()> {
        // this will internally attempt to send the tcp packet.
        let mut splits = self.splits.write().await;

        if let Err(_) = Self::send_packet(&self.socket, &mut splits, message).await {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Write Error",
            ));
        }

        return Ok(());
    }

    async fn recv(&mut self) -> Result<protocol::skyline::SkylinePacket, std::io::Error> {
        let packet = self.net_rx.recv().await;
        if let Some(packet) = packet {
            return Ok(packet);
        }

        return Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "Channel closed",
        ));
    }
}
