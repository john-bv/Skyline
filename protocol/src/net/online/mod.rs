use binary_util::{BinaryIo, types::varu32};

pub mod dataset;
pub mod ack;

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
    DataSet(dataset::DataSet),
    /// Used to recover lost packets.
    Ack(ack::AckVariant),
}

/// This is a generic ping packet.
/// It is used to measure the latency between the client and the server.
#[derive(BinaryIo)]
pub struct Ping {
    /// The time the packet was sent.
    pub send: u64
}

/// This is a generic pong packet.
#[derive(BinaryIo)]
pub struct Pong {
    /// Payload from the ping packet.
    pub send: u64,
    /// The time the packet was recieved by the peer.
    pub recv: u64
}

use super::types::LoginResponseCode;

/// This is the first packet sent by the client to the server.
/// It contains the identifier the client would like to refer to itself as:
///
/// IE: "EU", or "NA"
#[derive(BinaryIo)]
pub struct LoginPacket {
    /// A unique name for the client.
    /// If the name is already taken, the server will append a number to the end of the name.
    /// (only if the server is not in strict mode)
    pub name: String,
    /// The token of the Session, this is issued by the proxy for the client to use (security).
    /// If this is disabled, you can use the GUEST_UUID constant.
    pub token: String,
    /// A unique list of identifiers that the client has. These can be used to identify the client.
    /// This is the same as the name field, execpt it applies to all identifiers in this list.
    pub identifiers: Vec<String>,
}

/// The response for a login packet.
#[derive(BinaryIo)]
pub struct LoginResponse {
    /// The response from the server.
    pub response: LoginResponseCode,
    #[satisfy(self.response == LoginResponseCode::AccessGranted || self.response == LoginResponseCode::AccessLimited)]
    pub meta: Option<LoginResponseMeta>
}

#[derive(BinaryIo)]
pub struct LoginResponseMeta {
    /// The name the identifier the server has assigned to the client.
    /// This is unique and will always point to the same client.
    pub id: varu32,
    /// The name the server has assigned to the client.
    /// This is unique and will always point to the same client.
    pub name: String,
    /// This is the list sent by the client, any missing identifiers were rejected by the server
    /// please note the server DOES allow duplicates, so if you send "EU-1" here, you will get "EU-2" back.
    /// If EU-1 is already taken.
    pub identifiers: Vec<String>,
}