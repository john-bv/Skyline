use std::{sync::Arc, net::SocketAddr};

use tokio::{sync::{Mutex, mpsc::{Receiver, Sender}, Notify}, task::JoinHandle};

pub(crate) type ConnNetChan = Arc<Mutex<Receiver<Vec<u8>>>>;

pub enum ConnState {
    Offline,
    Connecting,
    Connected,
    TimingOut,
    Disconnected
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
    network_recv: ConnNetChan,
    disconnect: Arc<Notify>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>
}

impl Conn {
    pub async fn new(
        addr: SocketAddr,
        socket: &Arc<tokio::net::UdpSocket>,
        cleanup: Arc<Sender<SocketAddr>>,
        mtu: u16
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
}