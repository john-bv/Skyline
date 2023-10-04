use binary_util::{BinaryIo, types::varu32};

/// This packet can be sent by the server or by the client.
/// Both represent the same thing, but the server will send this packet
/// to the client when the client should gracefully disconnect.
///
/// The client will send this packet to the server when the client
/// wants to disconnect from the server.
#[derive(BinaryIo)]
pub struct Disconnect {
    pub reason: DisconnectReason,
}

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
    pub meta: Option<LoginResponseMeta>,
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


#[derive(Debug, Clone, Copy, BinaryIo, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoginResponseCode {
    /// An unknown issue occurred during packet processing, or the
    /// proxy is refusing the session
    Disconnect,
    /// The token provided was valid but either banned or expired.
    /// The server will stop processing packets from the client.
    DisconnectToken,
    /// The name provided was invalid, or overrides a reserved name.
    DisconnectName,
    /// The session is not allowed because the name or an identifier is not unique
    /// and already connected. (this is returned if the server is in "strict" mode)
    DisconnectDuplicate,
    /// The server allowed the session to connect, and the token is verified.
    AccessGranted,
    /// The server allowed the session to connect,
    /// but the token was invalid and the session is in guest mode.
    AccessLimited,
}

#[derive(BinaryIo)]
#[repr(u8)]
pub enum DisconnectReason {
    /// The connection was closed by the server.
    Closed,
    /// The connection was closed by the client.
    Disband,
    /// The token for th eclient is no longer valid.
    InvalidToken,
    InvalidName,
    InvalidIdentifiers,
    InvalidProtocol,
}
