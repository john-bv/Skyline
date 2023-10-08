use std::sync::Arc;

use crate::net::ConnAdapter;
#[cfg(feature = "udp")]
use crate::net::udp::conn::Conn;
#[cfg(feature = "tcp")]
use crate::net::tcp::conn::Conn;

pub struct Peer {
    inner: Arc<dyn ConnAdapter>,
}

impl Peer {
    pub fn new(inner: Arc<dyn ConnAdapter>) -> Self {
        Self {
            inner,
        }
    }
}