use binary_util::{BinaryIo, interfaces::{Writer, Reader}};

/// This is the "magic" or protocol identifier for the ZEQ protocol.
pub const SKYLINE_HEADER: &[u8] = b"SKYLINE_1.0.0";

/// This is the header that is sent by the server to the client.
pub struct SkylineHeader {}

impl Writer for SkylineHeader {
    fn write(&self, buf: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        buf.write(SKYLINE_HEADER)?;
        Ok(())
    }
}

impl Reader<SkylineHeader> for SkylineHeader {
    fn read(buf: &mut binary_util::ByteReader) -> Result<SkylineHeader, std::io::Error> {
        let mut header = [0u8; 16];
        buf.read(&mut header)?;
        if header != *SKYLINE_HEADER {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid header"));
        }
        Ok(SkylineHeader {})
    }
}

#[derive(BinaryIo)]
#[repr(u8)]
pub enum OfflinePackets {
    Disconnect(Disconnect) = 1,
    Ping(Ping),
    Pong(Pong),
    ConnectRequest(ConnectRequest),
    ConnectResponse(ConnectResponse)
}

/// This packet can be sent by the server or by the client.
/// Both represent the same thing, but the server will send this packet
/// to the client when the client should gracefully disconnect.
///
/// The client will send this packet to the server when the client
/// wants to disconnect from the server.
#[derive(BinaryIo)]
pub struct Disconnect {
    pub reason: DisconnectReason
}

#[derive(BinaryIo)]
#[repr(u8)]
pub enum DisconnectReason {
    InvalidToken,
    InvalidName,
    InvalidIdentifiers,
    InvalidProtocol
}

/// An offline ping packet.
/// This is used to measure the latency between the client and the server.
#[derive(BinaryIo)]
pub struct Ping {
    /// The time the packet was sent.
    pub send: u64
}

/// An offline pong packet.
#[derive(BinaryIo)]
pub struct Pong {
    /// Payload from the ping packet.
    pub send: u64,
    /// The time the packet was recieved by the peer.
    pub recv: u64
}

/// Attempts to establish a connection with the server.
/// This is the first packet sent by the client to the server.
#[derive(BinaryIo)]
pub struct ConnectRequest {
    pub header: SkylineHeader,
    /// The mtu you want to use as the client.
    pub mtu: u16,
    /// The current epoch in seconds on the client.
    pub client_time: u64,
}

/// The response for a connect request.
/// This is sent by the server to the client and allows the conenction
/// to be established.
#[derive(BinaryIo)]
pub struct ConnectResponse {
    pub header: SkylineHeader,
    /// The mtu the server will use for this client.
    pub mtu: u16,
    /// The current epoch in seconds on the server.
    pub server_time: u64,
    /// The time sent in the previous packet (ConnectRequest).
    pub client_time: u64
}