#[cfg(feature = "udp")]
pub mod udp;
#[cfg(feature = "udp")]
use self::udp::*;
#[cfg(feature = "tcp")]
pub mod tcp;

use async_trait::async_trait;
use protocol::skyline::connection::DisconnectReason;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ListenerState {
    Ready,
    Running,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ConnState {
    Offline,
    Connecting,
    Connected,
    TimingOut,
    Disconnected,
}

#[async_trait]
pub trait ConnAdapter {
    /// Returns the current state of the connection.
    /// Closes the connection forcefully, the connection is assumed to be disbanded after this.
    async fn close(&mut self, reason: DisconnectReason) -> std::io::Result<()>;
    /// Sends a skyline packet to the connection.
    async fn send(&self, packet: &protocol::skyline::SkylinePacket) -> std::io::Result<()>;
    /// Recieves a skyline packet from the connection.
    /// This function will block until a packet is recieved.
    async fn recv(&mut self) -> Result<protocol::skyline::SkylinePacket, std::io::Error>;
    /// Sends an arbitrary message to the connection.
    async fn send_message(&self, data: protocol::net::tcp::Messages) -> std::io::Result<()>;
}
