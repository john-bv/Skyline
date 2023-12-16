use protocol::skyline::SkylinePacket;
use std::sync::Arc;

use crate::net::ConnAdapter;

pub struct Peer {
    inner: Box<dyn ConnAdapter>,
}

impl Peer {
    pub fn new(inner: Box<dyn ConnAdapter>) -> Self {
        Self { inner }
    }

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
}
