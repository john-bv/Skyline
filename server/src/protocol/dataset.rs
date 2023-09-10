use std::ops::BitAnd;

/// This functions similarily to RakNet in that it is a reliable, ordered, and sequenced protocol.
/// However this is a more simplified, lightweight, and faster protocol.
/// Each dataset contains the following:
/// - A 1 byte header that contains the type of the dataset.
/// - A variable length payload that contains the data for the dataset.

use binary_util::BinaryIo;
use binary_util::types::varu32;

/// This is the "magic" or protocol identifier for the ZEQ protocol.
pub const ZEQA_DISPATCH_HEADER: &[u8] = b"ZEQA_DISPATCH:1.0.0";

/// Identifies the type of the dataset.
#[derive(Debug, Clone, Copy, BinaryIo, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DataBits {
    /// The packet is split into multiple packets.
    Split = 0b0000_0001,
    /// The packet is reliable.
    /// We need to wait for an acknowledgement from the recipient.
    Reliable = 0b0000_0010,
    /// The packet is Ordered.
    /// If this is the case, we must recieve all packets before we can process them.
    Ordered = 0b0000_0100,
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
    pub total: u16,
    pub index: u16
}

/// The information to order packets correctly.
/// This is primarily used when performing db queries.
#[derive(Debug, Clone, BinaryIo)]
pub struct OrderInfo {
    pub id: u16,
    pub index: u16
}

#[derive(Debug, Clone, BinaryIo)]
pub struct DataSet {
    pub flags: DataBits,
    /// The sequence number of the packet.
    /// This is used to discard duplicate/old packets.
    pub seq: varu32,
    /// If the flags contain the `Split` flag, this will contain the information
    #[satisfy((self.flags & DataBits::Split) != 0)]
    pub split: Option<SplitInfo>,
    /// If the flags contain the `Ordered`, the order information will be here.
    #[satisfy((self.flags & DataBits::Ordered) != 0)]
    pub order: Option<OrderInfo>,
    /// the payload, this is prefixed by a varu32 length by binary_util.
    pub payload: Vec<u8>
}