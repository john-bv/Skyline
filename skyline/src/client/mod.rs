/// A client is a peer that connects to a skyline instance.
/// This is anything that you would consider as a "client",
/// maybe a gameserver, api, or website that needs all need to communicate
/// with each other.
pub mod client;
/// A node is like a client, except it is hosting it's own protocol.
/// It's a self operating client that controls it's own channels, and
/// authentications.
///
/// This is useful for something like a bot, or a database, where you might
/// want to have a node that is always online, and can be interacted with
/// but only when you want to.
///
/// The primary reason for separating nodes from clients is to isolate the
/// implementation of a "service" over a "client", where a service is something
/// that is always online, and a client is like a customer visiting a website, where
/// they can come and go as they please.
pub mod node;

pub use client::Client;
pub use node::Node;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ConnectionNetworkMode {
    Tcp,
    Udp
}