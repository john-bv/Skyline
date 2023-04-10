use binary_utils::io::{ByteReader, ByteWriter};
use binary_utils::interfaces::{Reader, Writer};
use crate::protocol::PacketId;

use crate::packet_id;

/// This is the first packet sent by the client to the server.
/// It contains the identifier the client would like to refer to itself as:
///
/// IE: "EU", or "NA"
pub struct LoginPacket {
    pub identifier: String,
    pub token: String
}
packet_id!(LoginPacket, 0x01);

impl Writer for LoginPacket {
    fn write(&self, writer: &mut ByteWriter) -> Result<(), std::io::Error> {
        writer.write_string(&self.identifier.clone())?;
        writer.write_string(&self.token.clone())?;
        Ok(())
    }
}

impl Reader<LoginPacket> for LoginPacket {
    fn read(reader: &mut ByteReader) -> Result<Self, std::io::Error> {
        let identifier = reader.read_string()?;
        let token = reader.read_string()?;
        Ok(Self { identifier, token })
    }
}

/// This packet can be sent by the server or by the client.
/// Both represent the same thing, but the server will send this packet
/// to the client when the client should gracefully disconnect.
///
/// The client will send this packet to the server when the client
/// wants to disconnect from the server.
pub struct Disconnect {
    pub reason: String
}
packet_id!(Disconnect, 0x02);

impl Writer for Disconnect {
    fn write(&self, writer: &mut ByteWriter) -> Result<(), std::io::Error> {
        writer.write_string(&self.reason.clone())?;
        Ok(())
    }
}

impl Reader<Disconnect> for Disconnect {
    fn read(reader: &mut ByteReader) -> Result<Self, std::io::Error> {
        let reason = reader.read_string()?;
        Ok(Self { reason })
    }
}

// todo: Use blowfish encryption