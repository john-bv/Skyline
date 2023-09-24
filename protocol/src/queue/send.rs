use std::{collections::HashMap, sync::Arc};

use crate::net::online::dataset::DataSet;
use super::{split::{SplitQueueError, SplitQueue}, recovery::RecoveryQueue, ord::OrdQueue};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SendQueueError {
    /// The packet is too large to be sent, try sending a smaller packet.
    PacketTooLarge,
    /// There was an error when splitting the packet.
    SplitError(SplitQueueError),
    /// Could not send the packet.
    SendError
}

pub struct SendQueue {
    mtu_size: u16,
    /// The current sequence number.
    /// This is incremented every time a packet is sent.
    seq: u32,
    ack: RecoveryQueue<DataSet>,
    splitq: SplitQueue,
    /// channels
    /// (channel, seq)
    ord_chans: HashMap<u16, u32>,
    /// The packets we're ready to process
    queue: Vec<DataSet>,
}