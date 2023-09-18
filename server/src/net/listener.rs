use std::{net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};

use tokio::sync::Notify;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::utils::PossiblySocketAddr;
use super::conn::Conn;

pub(crate) type ConnMap = Arc<Mutex<HashMap<SocketAddr, Conn>>>;

pub struct Listener {
    pub addr: SocketAddr,
    /// This is a notifier to kill the listener and all of it's tasks.
    /// When this is notified, the listener will close.
    close_notifier: Arc<Notify>,
    /// A mpsc channel that will send connections to the user and back to the listener.
    rx_accept_channel: Receiver<()>,
    tx_accept_channel: Sender<()>,
    /// This is a hash_map of all connections, it contains a buffer channel
    /// that will send data to the connection.
    connections: ConnMap,
}

impl Listener {
    pub async fn bind<I: for<'a> Into<PossiblySocketAddr<'a>>>(
        address: I,
    ) -> Result<Self, std::io::Error> {
        let addr = (address.into() as PossiblySocketAddr).to_socket_addr();
        let close_notifier = Arc::new(Notify::new());

        let (tx_accept_channel, rx_accept_channel) = tokio::sync::mpsc::channel::<()>(5);
        let connections = Arc::new(Mutex::new(HashMap::<SocketAddr, Conn>::new()));


        if let None = addr {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Bind address is invalid."
            ));
        }

        Ok(
            Self {
                addr: addr.unwrap(),
                close_notifier,
                rx_accept_channel,
                tx_accept_channel,
                connections
            }
        )
    }

    pub async fn close(&self) {
        self.close_notifier.notify_waiters();
    }
}