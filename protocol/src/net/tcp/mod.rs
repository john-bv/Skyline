use binary_util::{
    interfaces::{Reader, Writer},
    BinaryIo,
};
use std::io;

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone)]
pub struct Frame {
    pub id: u16,
    pub message: Vec<u8>,
}

impl Frame {
    pub fn new(message: Vec<u8>) -> Self {
        Self {
            id: 54,
            message: message,
        }
    }
}

impl Reader<Frame> for Frame {
    fn read(buf: &mut binary_util::ByteReader) -> Result<Frame, std::io::Error> {
        let id = buf.read_u16()?;

        if id != 54 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid frame ID"));
        }

        let message = buf.read_type::<Vec<u8>>()?;

        Ok(Frame { id, message })
    }
}

impl Writer for Frame {
    fn write(&self, buf: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        buf.write_u16(self.id)?;
        buf.write_type(&self.message)?;

        Ok(())
    }
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum Messages {
    Connect(Connect) = 0,
    Hello(Hello) = 1,
    Disconnect(Disconnect) = 2,
    HeartbeatAck(HeartbeatAck) = 3,
    SplitPacket(SplitPacket) = 6,
    SplitOk(SplitOk) = 7,
    Payload(Payload) = 8,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct Connect {
    /// skyline protocol version
    /// 0x0001 = 1
    /// 0x0002 = 2 etc
    pub version: u16,
    /// max packet size
    /// by default this is 1024
    /// > CURRENTLY IGNORED
    pub max_size: u16,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct Hello {
    pub timestamp: Option<u64>,
    pub interval: u16,
}

#[derive(Debug, Clone, BinaryIo, PartialEq, Copy)]
#[repr(u8)]
pub enum Disconnect {
    NotAuthorized = 0,
    InvalidCredentials,
    InvalidProtocol,
    SelfInitiated,
    Unknown,
}

impl std::fmt::Display for Disconnect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disconnect::NotAuthorized => write!(f, "Not authorized"),
            Disconnect::InvalidCredentials => write!(f, "Invalid credentials"),
            Disconnect::InvalidProtocol => write!(f, "Invalid protocol"),
            Disconnect::SelfInitiated => write!(f, "Self initiated"),
            Disconnect::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, BinaryIo)]
pub struct Heartbeat {
    pub interval: u64,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct HeartbeatAck {
    pub timestamp: u64,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct SplitPacket {
    pub id: u16,
    pub index: u16,
    pub size: u16,
    pub data: Vec<u8>,
}

impl SplitPacket {
    pub fn split(id: u16, data: &[u8]) -> io::Result<Vec<Self>> {
        let max_mtu: usize = 1024 - 60 - 12; // 60 = TCP, 12 = Proto overhead

        if data.len() > max_mtu.into() {
            let splits = data.chunks(max_mtu.into());
            let parts: u16 = splits.len().try_into().unwrap();

            let mut packets = Vec::new();

            for (i, split) in splits.enumerate() {
                packets.push(Self {
                    id,
                    index: i as u16,
                    size: parts,
                    data: split.to_vec(),
                });
            }

            Ok(packets)
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Data is too small to split",
            ));
        }
    }
}

#[derive(Debug, Clone, BinaryIo)]
pub struct SplitOk {
    pub id: u16,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct Payload {
    pub data: Vec<u8>,
}
