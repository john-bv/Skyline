#[cfg(feature = "udp")]
pub mod udp;
use std::net::SocketAddr;

#[cfg(feature = "udp")]
use self::udp::*;
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
pub trait ConnAdapter: Send + Sync {
    /// Returns the current state of the connection.
    /// Closes the connection forcefully, the connection is assumed to be disbanded after this.
    async fn close(&mut self, reason: DisconnectReason) -> std::io::Result<()>;
    /// Sends a skyline packet to the connection.
    async fn send(&self, packet: &protocol::skyline::SkylinePacket) -> std::io::Result<()>;
    /// Recieves a skyline packet from the connection.
    /// This function will block until a packet is recieved.
    async fn recv(&mut self) -> std::io::Result<protocol::skyline::SkylinePacket>;
    /// Sends an arbitrary message to the connection.
    // todo: REMOVE THIS
    async fn send_message(&self, data: protocol::net::tcp::Messages) -> std::io::Result<()>;
    /// This will get the address of the connection.
    /// While not required, it is recommended to use this as the address of the connection.
    fn get_addr(&self) -> SocketAddr;
}

/// Trait is responsible for interfacing with the Server, each listener is required
/// to implement this trait.
///
/// It provides a universal API that can be used by the server without having to
/// worry about the underlying implementation.
#[async_trait]
pub trait NetworkInterface {
    // Returns the current state of the listener.
    // This will do all binding and setup required for the listener to be ready.
    // async fn new(addr: &str) -> std::io::Result<Self>;
    async fn new(addr: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    /// Binds the listener to the specified address.
    /// This will do all binding and setup required for the listener to be ready.
    async fn bind(&mut self) -> std::io::Result<()>;

    /// Accepts a new connection from the listener.
    /// The connection is passed on to the caller
    async fn accept(&mut self) -> std::io::Result<Box<dyn ConnAdapter>>;

    /// Closes the listener forcefully, the listener is assumed to be disbanded after this.
    /// This will close all connections associated with the listener.
    async fn close(&mut self) -> std::io::Result<()>;

    fn get_name(&self) -> &str {
        "null"
    }
}


pub struct NullInterface;

#[async_trait]
impl NetworkInterface for NullInterface {
    async fn new(_addr: &str) -> std::io::Result<Self> {
        Ok(Self)
    }

    async fn bind(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    async fn accept(&mut self) -> std::io::Result<Box<dyn ConnAdapter>> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "NullInterface does not accept connections",
        ))
    }

    async fn close(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}