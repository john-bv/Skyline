use std::sync::Arc;

#[cfg(feature = "tcp")]
use crate::net::tcp::conn::Conn;
#[cfg(feature = "udp")]
use crate::net::udp::conn::Conn;
use crate::net::ConnAdapter;

pub struct Peer {
    inner: Conn,
}

impl Peer {
    pub fn new(inner: Conn) -> Self {
        Self { inner }
    }

    pub async fn close(
        &mut self,
        reason: protocol::skyline::connection::DisconnectReason,
    ) -> std::io::Result<()> {
        self.inner.close(reason).await?;
        Ok(())
    }
}
