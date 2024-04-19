
/// This is a struct responsible for dispatching between clients and getting information like
/// the amount of clients connected, and db stuff.
pub struct PeerManager {
    peers: HashMap<PeerId, Arc<Peer>>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub async fn add_peer(&mut self, peer: Arc<Peer>) -> Result<(), &'static str> {
        if self.peers.contains_key(&peer.id) {
            let _ = peer
                .close(protocol::skyline::connection::DisconnectReason::Conflict)
                .await;
            return Err("Peer already exists");
        }

        self.peers.insert(peer.id, peer);
        Ok(())
    }

    pub fn remove_peer(&mut self, peer: Arc<Peer>) {
        // when a peer is removed, we should close the connection
        // todo: determine whether or not we should await the close
        let _ = tokio::runtime::Handle::current().block_on(
            peer.inner
                .close(protocol::skyline::connection::DisconnectReason::Closed),
        );
        self.peers.remove(&peer.id);
    }

    pub fn get_next_id(&self) -> PeerId {
        let mut id = 0;
        for peer in &self.peers {
            if peer.1.id > id {
                id = peer.1.id;
            }
        }
        id + 1
    }

    /// Dispatches a packet to all peers
    /// This is a blocking call, and should be called from a tokio task.
    pub async fn dispatch(&mut self, packet: &SkylinePacket) {
        for peer in self.peers.values() {
            let _ = peer.send_raw(packet).await;
        }
    }
}

impl Iterator for PeerManager {
    type Item = Peer;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("Not implemented")
    }
}
