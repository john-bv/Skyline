use protocol::skyline::SkylinePacket;
use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use tokio::sync::Notify;

use crate::net::ConnAdapter;

/// This is a struct responsible for dispatching between clients and getting information like
/// the amount of clients connected, and db stuff.
pub struct PeerManager {
    peers: HashMap<PeerId, RwLock<Peer>>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self { peers: HashMap::new() }
    }

    pub async fn add_peer(&mut self, peer: Peer) -> Result<(), &'static str> {
        if self.peers.contains_key(&peer.id) {
            peer.close(protocol::skyline::connection::DisconnectReason::Conflict).await;
            return Err("Peer already exists");
        }

        self.peers.insert(peer.id, RwLock::new(peer));
        Ok(())
    }

    pub fn remove_peer(&mut self, peer: Peer) {
        let mut inner = peer.inner.lock().unwrap();
        // when a peer is removed, we should close the connection
        tokio::runtime::Handle::current().block_on(inner.close(protocol::skyline::connection::DisconnectReason::Closed));
        self.peers
            .remove(&peer.id);
    }

    pub fn get_next_id(&self) -> PeerId {
        let mut id = 0;
        for peer in &self.peers {
            if peer.1.read().unwrap().id > id {
                id = peer.1.read().unwrap().id;
            }
        }
        id + 1
    }

    pub fn dispatch(&mut self, packet: &SkylinePacket) {
        panic!("Not implemented")
    }
}

impl Iterator for PeerManager {
    type Item = Peer;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("Not implemented")
    }
}

pub type PeerId = usize;

pub struct Peer {
    inner: Arc<Mutex<dyn ConnAdapter>>,
    closer: Arc<Notify>,
    id: PeerId,
}

impl Peer {
    pub async fn close(
        &self,
        reason: protocol::skyline::connection::DisconnectReason,
    ) -> std::io::Result<()> {
        let mut adapter = self.inner.lock().unwrap();
        adapter.close(reason).await?;
        Ok(())
    }

    pub async fn send_raw(&mut self, packet: &SkylinePacket) -> std::io::Result<()> {
        let adapter = self.inner.lock().unwrap();
        adapter.send(packet).await?;
        Ok(())
    }

    pub async fn init(inner: Arc<Mutex<dyn ConnAdapter>>, closer: Arc<Notify>, id: PeerId) -> Self {
        Self { inner, closer, id: 0 }
    }
}
