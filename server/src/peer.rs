use std::sync::Arc;

#[cfg(feature = "udp")]
use crate::net::udp::conn::Conn;
#[cfg(feature = "tcp")]
use crate::net::tcp::conn::Conn;

pub struct Peer {
    base: Arc<Conn>,
}

impl Peer {

}