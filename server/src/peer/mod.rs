use protocol::skyline::SkylinePacket;
use std::sync::Arc;
use tokio::sync::Notify;

use crate::net::ConnAdapter;

/// This is a struct responsible for dispatching between clients and getting information like
/// the amount of clients connected, and db stuff.
pub struct PeerManager {
    peers: Vec<Box<Peer>>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self { peers: Vec::new() }
    }

    pub fn add_peer(&mut self, peer: Box<Peer>) {
        self.peers.push(peer);
    }

    pub fn remove_peer(&mut self, peer: Box<Peer>) {
        self.peers
            .retain(|p| p.inner.get_addr_token() != peer.inner.get_addr_token());
    }

    pub fn get_peers(&self) -> &Vec<Box<Peer>> {
        &self.peers
    }

    pub fn dispatch(&mut self, packet: &SkylinePacket) {
        panic!("Not implemented")
    }
}

pub struct Peer {
    inner: Box<dyn ConnAdapter>,
    closer: Arc<Notify>,
}

impl Peer {
    pub async fn close(
        &mut self,
        reason: protocol::skyline::connection::DisconnectReason,
    ) -> std::io::Result<()> {
        self.inner.close(reason).await?;
        Ok(())
    }

    pub async fn send_raw(&mut self, packet: &SkylinePacket) -> std::io::Result<()> {
        self.inner.send(packet).await?;
        Ok(())
    }

    pub async fn init(inner: Box<dyn ConnAdapter>, closer: Arc<Notify>) -> Self {
        Self { inner, closer }
    }
}
