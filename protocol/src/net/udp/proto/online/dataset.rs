use std::ops::BitAnd;

use binary_util::interfaces::{Reader, Writer};
use binary_util::types::varu32;
/// This functions similarly to RakNet in that it is a reliable, ordered, and sequenced protocol.
/// However this is a more simplified, lightweight, and faster protocol.
/// Each dataset contains the following:
/// - A 1 byte header that contains the type of the dataset.
/// - A variable length payload that contains the data for the dataset.
use binary_util::BinaryIo;

use crate::net::udp::types::sized_vec::SizedVec;

#[derive(Debug, Clone)]
pub struct DataBit {
    bit: u8,
}

impl DataBit {
    pub fn new() -> Self {
        Self { bit: 0 }
    }

    pub fn with_reliable(mut self) -> Self {
        self.bit |= DataBits::Reliable as u8;
        self
    }

    pub fn with_ordered(mut self) -> Self {
        self.bit |= DataBits::Ordered as u8;
        self
    }

    pub fn with_split(mut self) -> Self {
        self.bit |= DataBits::Split as u8;
        self
    }

    pub fn with_unreliable(mut self) -> Self {
        self.bit |= DataBits::Unreliable as u8;
        self
    }

    pub fn is_reliable(&self) -> bool {
        (self.bit & DataBits::Reliable as u8) != 0
    }

    pub fn is_ordered(&self) -> bool {
        (self.bit & DataBits::Ordered as u8) != 0
    }

    pub fn is_split(&self) -> bool {
        (self.bit & DataBits::Split as u8) != 0
    }

    pub fn is_unreliable(&self) -> bool {
        (self.bit & DataBits::Unreliable as u8) != 0
    }

    pub fn get(&self) -> u8 {
        self.bit
    }
}

impl Reader<DataBit> for DataBit {
    fn read(buf: &mut binary_util::ByteReader) -> Result<DataBit, std::io::Error> {
        let bit = buf.read_u8()?;
        Ok(Self { bit })
    }
}

impl Writer for DataBit {
    fn write(&self, buf: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        buf.write_u8(self.bit)?;
        Ok(())
    }
}

/// Identifies the type of the dataset.
#[derive(Debug, Clone, Copy, BinaryIo, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DataBits {
    /// The packet is split into multiple packets.
    Split = 0b0000_0001,
    /// The packet is reliable.
    /// We need to wait for an acknowledgement from the recipient.
    /// If this isn't included, we can assume it's unreliable.
    Reliable = 0b0000_0010,
    /// The packet is Ordered.
    /// If this is the case, we must recieve all packets before we can process them.
    Ordered = 0b0000_0100,
    /// The packet is unreliable.
    /// Default bit
    Unreliable = 0b0000_1000,
}

impl DataBits {
    pub fn is_split(&self) -> bool {
        (*self & DataBits::Split) != 0
    }

    pub fn is_reliable(&self) -> bool {
        (*self & DataBits::Reliable) != 0
    }

    pub fn is_ordered(&self) -> bool {
        (*self & DataBits::Ordered) != 0
    }

    pub fn is_unreliable(&self) -> bool {
        (*self & DataBits::Unreliable) != 0
    }
}

impl BitAnd for DataBits {
    type Output = u8;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 & rhs as u8) }
    }
}

/// The information necessary to retain when splitting a packet.
/// This is used to reassemble the packet on the other end.
#[derive(Debug, Clone, BinaryIo)]
pub struct SplitInfo {
    pub id: u16,
    pub total: u32,
    pub index: u32,
}

/// The information to order packets correctly.
/// This is primarily used when performing db queries.
#[derive(Debug, Clone, BinaryIo)]
pub struct OrderInfo {
    pub id: u16,
    pub index: u32,
    pub sequence: u32,
}

/// A datagram is a collection of datasets.
/// This is the main packet that is sent over the network.
///
/// Visual representation:
///```md
///                  ┌──────────┐
///                  │ Datagram │
///                  └──────────┘
///                        │
///                       / \
///                      /   \
///      ┌───────────┐┌──────────┐┌──────────┐
/// ┌────┴────┐ ┌────┴┴───┐ ┌────┴┴───┐ ┌────┴────┐
/// │   SET   │ │   SET   │ │   SET   │ │   SET   │
/// └─────────┘ └─────────┘ └─────────┘ └─────────┘
/// ```
#[derive(Debug, Clone, BinaryIo)]
pub struct Datagram {
    pub sequence: u32,
    pub sets: Vec<DataSet>,
}

impl Datagram {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            sets: Vec::new(),
        }
    }

    pub fn with_sequence(mut self, sequence: u32) -> Self {
        self.sequence = sequence;
        self
    }

    pub fn with_sets(mut self, sets: Vec<DataSet>) -> Self {
        self.sets = sets;
        self
    }

    pub fn with_set(mut self, set: DataSet) -> Self {
        self.sets.push(set);
        self
    }

    pub fn push_set(&mut self, set: DataSet) {
        self.sets.push(set);
    }
}

#[derive(Debug, Clone, BinaryIo)]
pub struct DataSet {
    pub flags: DataBit,
    /// The sequence number of the packet.
    /// This is used to discard duplicate/old packets.
    pub seq: varu32,
    /// The reliable sequence number
    #[satisfy(self.flags.is_reliable())]
    pub reliable_seq: Option<varu32>,
    /// If the flags contain the `Split` flag, this will contain the information
    #[satisfy(self.flags.is_split())]
    pub split: Option<SplitInfo>,
    /// If the flags contain `Ordered`, the order information will be here.
    #[satisfy(self.flags.is_ordered())]
    pub order: Option<OrderInfo>,
    /// the payload, this is prefixed by a varu32 length by binary_util.
    pub payload: SizedVec<u16, u8>,
}

impl DataSet {
    pub fn new() -> Self {
        Self {
            flags: DataBit::new(),
            seq: 0.into(),
            reliable_seq: None,
            split: None,
            order: None,
            payload: SizedVec::new(0),
        }
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = SizedVec::new(0);
        self.payload.data = payload;
        self
    }

    pub fn with_reliable(mut self, seq: u32) -> Self {
        self.flags = self.flags.with_reliable();
        self.reliable_seq = Some(seq.into());
        self
    }

    pub fn with_bits(mut self, bits: DataBit) -> Self {
        self.flags = bits;
        self
    }

    pub fn with_ordered(mut self, id: u16, index: u32) -> Self {
        self.flags = self.flags.with_ordered();
        self.order = Some(OrderInfo {
            id,
            index,
            sequence: 0,
        });
        self
    }

    pub fn is_split(&self) -> bool {
        self.split.is_some()
    }
}
