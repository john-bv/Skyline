use binary_util::BinaryIo;

/// This is the first packet sent by the client to the server.
/// It contains the identifier the client would like to refer to itself as:
///
/// IE: "EU", or "NA"
#[derive(BinaryIo)]
pub struct LoginPacket {
    /// A unique identifier for the client.
    /// If the name is already taken, the server will append a number to the end of the name.
    pub id: String,
    /// The token of the Session, this is issued by the proxy for the client to use (security).
    /// If this is disabled, you can use the GUEST_UUID constant.
    pub token: String
    /// A unique 
}

/// This packet can be sent by the server or by the client.
/// Both represent the same thing, but the server will send this packet
/// to the client when the client should gracefully disconnect.
///
/// The client will send this packet to the server when the client
/// wants to disconnect from the server.
#[derive(BinaryIo)]
pub struct Disconnect {
    pub reason: String
}