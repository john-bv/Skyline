use std::{net::SocketAddr, sync::Arc};
use std::io::{BufReader, BufRead};

use async_trait::async_trait;
use binary_util::interfaces::Writer;
use protocol::skyline::{connection::{DisconnectReason, Disconnect}, SkylinePacket};
use tokio::{sync::{Notify, RwLock}, io::AsyncWriteExt};

use crate::net::{ConnState, ConnAdapter};

pub struct Conn<'a> {
    pub addr: SocketAddr,
    pub state: ConnState,
    close_notifier: Arc<Notify>,
    /// Single channel for digesting skyline packets.
    net_rx: tokio::sync::mpsc::Receiver<SkylinePacket>,
    /// Contrary to the name, this is a DIFFERENT channel directly for sending
    /// packets to the peer/client.
    net_tx: tokio::sync::mpsc::Sender<&'a [u8]>,
}

impl Conn<'_> {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        // initialize new notifier;
        let close_notifier = Arc::new(Notify::new());

        // move a dedicated task to handle the stream.
        let (net_tx, mut net_rx) = tokio::sync::mpsc::channel::<&[u8]>(100);
        let (pak_tx, mut pak_rx) = tokio::sync::mpsc::channel::<SkylinePacket>(100);

        tokio::spawn(async move {
            // loop until closed or disconnect
            let mut buf: [u8; 1024] = [0; 1024];
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            loop {
                tokio::select! {
                    _ = close_notifier.notified() => {
                        break;
                    },
                    recv = stream.readable().await {
                        match stream.try_read(&mut buf) {
                            Ok(0) => {
                                // No data was read...
                                break;
                            },
                            Ok(n) => {
                                // read n bytes
                                byte_writer.write(&buf[..n]);
                    }
                }
            }
        });

    }
}

#[async_trait]
impl ConnAdapter for Conn<'_> {
    async fn close(&self, reason: DisconnectReason) -> Result<(), std::io::Result<()>> {
        self.close_notifier.notify_waiters();
        let disconnect = Disconnect {
            reason: DisconnectReason::InvalidProtocol
        };

        self.send(&SkylinePacket::Disconnect(disconnect)).await;
        Ok(())
    }

    async fn send(&self, packet: &SkylinePacket) -> Result<usize, std::io::Error> {
        let bytes = packet.write_to_bytes()?;
        let bytes = bytes.as_slice();
        if let Err(_) = self.net_tx.send(bytes).await {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Send Error"));
        }

        return Ok(bytes.len());
    }

    async fn recv(&self) -> Result<protocol::skyline::SkylinePacket, std::io::Error> {
        let packet = self.net_rx.recv().await;
        if let Some(packet) = packet {
            return Ok(packet);
        }

        return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Channel closed"));
    }
}