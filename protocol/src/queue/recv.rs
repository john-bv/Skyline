use std::collections::{HashMap, HashSet};

use crate::{
    net::online::{
        ack::{Acknowledgeable, Acknowledgement},
        dataset::{DataBits, DataSet},
    },
    MAX_SPLIT_SIZE,
};

use super::{ord::OrdQueue, split::SplitQueue, window::Window};

pub enum RecvQueueError {
    /// This an old sequence
    OldSequence,
    /// old reliable sequence
    OldReliableSequence,
    /// Too many splits within packet
    SplitSizeTooLarge,
    /// We are still missing some packets in the split
    MissingIndicesInSplit,
}

pub struct RecvQueue {
    split_q: SplitQueue,
    order_q: HashMap<u16, OrdQueue<Vec<u8>>>,
    /// Acks that we have acknowledged.
    // ack: HashMap<u32, u64>,
    //              seq, epoch - Removed because we don't need to know when we acked it.
    ack: HashSet<u32>,
    /// We don't have these packets..
    nack: HashSet<u32>,
    /// Packets that we have recieved.
    window: Window,
    reliable_window: Window,
    /// These are packets that need to be processed and have been fully extracted.
    queue: Vec<Vec<u8>>,
}

impl RecvQueue {
    pub fn new() -> Self {
        Self {
            split_q: SplitQueue::new(),
            order_q: HashMap::new(),
            ack: HashSet::new(),
            nack: HashSet::new(),
            window: Window::new(),
            reliable_window: Window::new(),
            queue: Vec::new(),
        }
    }

    pub fn insert(&mut self, data_set: DataSet) -> Result<(), RecvQueueError> {
        if !self.window.insert(data_set.seq.into()) {
            return Err(RecvQueueError::OldSequence);
        }

        if self.window.window().start < data_set.seq.into() {
            // this is a new packet, we might not have previous packets!
            (self.window.window().start..(data_set.seq.0))
                .into_iter()
                .for_each(|seq| {
                    self.nack.insert(seq);
                });
        }

        self.ack.insert(data_set.seq.into());

        self.process_data_set(&data_set)?;

        Ok(())
    }

    fn process_data_set(&mut self, data_set: &DataSet) -> Result<(), RecvQueueError> {
        if let Some(ref seq) = data_set.reliable_seq {
            if !self.reliable_window.insert(seq.0) {
                return Err(RecvQueueError::OldReliableSequence);
            }
        }
        if let Some(ref split) = data_set.split {
            if split.total > MAX_SPLIT_SIZE.into() {
                return Err(RecvQueueError::SplitSizeTooLarge);
            }

            if let Err(_) = self.split_q.insert(data_set.clone()) {}

            match self.split_q.join(split.id) {
                Ok(pk) => {
                    // we have the full packet!
                    self.queue.push(pk);
                    return Ok(());
                }
                Err(_) => {
                    // we're still missing some packets
                    return Err(RecvQueueError::MissingIndicesInSplit);
                }
            }
        }

        if data_set.flags.is_ordered() {
            if let Some(ref order_info) = data_set.order {
                let channel = self
                    .order_q
                    .entry(order_info.id)
                    .or_insert_with(|| OrdQueue::new());

                if let Ok(_) = channel.insert(order_info.index, data_set.payload.clone().data) {
                    // we have the packets in order now,
                    // we can push them to the queue.
                    channel.flush().into_iter().for_each(|pk| {
                        self.queue.push(pk);
                    });
                }
            }
        } else {
            self.queue.push(data_set.payload.clone().into());
        }

        Ok(())
    }
}

impl Acknowledgeable for RecvQueue {
    type NackItem = ();

    fn ack(&mut self, ack: Acknowledgement) {
        ack.seqs.into_iter().for_each(|seq| {
            self.ack.remove(&seq);
        });
    }
}
