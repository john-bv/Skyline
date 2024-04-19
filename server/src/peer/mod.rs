use protocol::skyline::SkylinePacket;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};
use tokio::{sync::Notify, task::JoinHandle};

use crate::net::ConnAdapter;

pub enum PeerState {
    /// The peer is connected, and is ready to recieve packets.
    Connected,
    /// the Peer is timing out, because it has not been sent a packet or has not sent a packet.
    /// This will cause the peer to close.
    TimingOut,
    /// The peer is disconnected, and being discarded. This is the final state of the peer before it
    /// is removed from the peer manager.
    Disconnected,
}

impl PeerState {
    fn is_connected(&self) -> bool {
        match self {
            PeerState::Connected | PeerState::TimingOut => true,
            _ => false,
        }
    }

    /// The logic here may be confusing, for TCP, all connections are considered
    /// reliable because it is handled within protocol, however for UDP, we need to
    /// check if the connection is reliable.
    ///
    /// So when using this in TCP mode, you will get UDP like behavior.
    fn is_reliable(&self) -> bool {
        match self {
            PeerState::Connected => true,
            _ => false,
        }
    }
}

pub type PeerId = usize;

pub struct Peer {
    pub state: Arc<Mutex<PeerState>>,
    inner: Arc<dyn ConnAdapter>,
    closer: Arc<Notify>,
    id: Arc<Mutex<PeerId>>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl Peer {
    pub async fn new(
        inner: Arc<dyn ConnAdapter>,
        closer: Arc<Notify>,
        id: PeerId,
    ) -> Self {
        let p = Self {
            state: Arc::new(Mutex::new(PeerState::Connected)),
            inner,
            closer: closer.clone(),
            id: Arc::new(Mutex::new(id)),
            tasks: Arc::new(Mutex::new(Vec::new()))
        };

        p.listen_for_close(closer).await;

        let tk = p.tasks.clone();
        let mut tasks = tk.lock().unwrap();

        // initailize the tasks, we need to listen for network traffic
        // and we also need to tick to make sure the connection is still alive
        return p;
    }

    /// Closes the peer connection.
    pub async fn close(
        &self,
        reason: protocol::skyline::connection::DisconnectReason,
    ) -> std::io::Result<()> {
        self.inner.close(reason).await?;
        self.closer.notify_waiters();
        Ok(())
    }

    /// Forwards a packet to the connection adapter.
    /// This will block until the packet is sent.
    pub async fn send_raw(&self, packet: &SkylinePacket) -> std::io::Result<()> {
        self.inner.send(packet).await?;
        Ok(())
    }

    async fn listen_for_close(&self, closer: Arc<Notify>) {
        let inner = self.inner.clone();
        tokio::task::spawn(async move {
            loop {
                closer.notified().await;
                let _ = inner.close(protocol::skyline::connection::DisconnectReason::Closed).await;
                break;
            }
        });
    }
}
