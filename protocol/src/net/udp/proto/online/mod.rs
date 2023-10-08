use binary_util::{types::varu32, BinaryIo};

pub mod ack;
pub mod dataset;

/// Online packets have differing ids from offline ones!
#[derive(BinaryIo)]
#[repr(u16)]
pub enum OnlinePackets {
    /// This is a generic ping packet.
    /// It is used to measure the latency between the client and the server.
    Ping(Ping),
    /// This is a generic pong packet.
    Pong(Pong),
    /// This is a generic packet that is used to send large data to the client.
    /// Think of this like the "gamewrapper" packet for RakNet.
    Datagram(dataset::Datagram),
    /// Used to recover lost packets.
    Ack(ack::AckVariant),
}

/// This is a generic ping packet.
/// It is used to measure the latency between the client and the server.
#[derive(BinaryIo)]
pub struct Ping {
    /// The time the packet was sent.
    pub send: u64,
}

/// This is a generic pong packet.
#[derive(BinaryIo)]
pub struct Pong {
    /// Payload from the ping packet.
    pub send: u64,
    /// The time the packet was recieved by the peer.
    pub recv: u64,
}
