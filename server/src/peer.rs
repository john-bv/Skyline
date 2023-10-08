use std::sync::Arc;

#[cfg(feature = "tcp")]
use crate::net::tcp::conn::Conn;
#[cfg(feature = "udp")]
use crate::net::udp::conn::Conn;
use crate::net::ConnAdapter;

pub struct Peer {
    inner: Arc<dyn ConnAdapter>,
}

impl Peer {
    pub fn new(inner: Arc<dyn ConnAdapter>) -> Self {
        Self { inner }
    }
}
