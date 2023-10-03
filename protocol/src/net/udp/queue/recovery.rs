use std::collections::HashMap;

use crate::util::current_epoch;

use super::{NetQueue, NetQueueError};

/// A recovery queue is used to store packets that need to be resent.
/// This is used for sequenced and ordered packets.
#[derive(Debug, Clone)]
pub struct RecoveryQueue<Item> {
    /// The current queue of packets by timestamp
    /// (seq, (packet, timestamp))
    queue: HashMap<u32, (u64, Item)>,
}

impl<Item> RecoveryQueue<Item>
where
    Item: Clone,
{
    pub fn new() -> Self {
        Self {
            queue: HashMap::new(),
        }
    }

    pub fn insert_id(&mut self, seq: u32, item: Item) {
        self.queue.insert(seq, (current_epoch(), item));
    }

    pub fn get_all(&mut self) -> Vec<(u32, Item)> {
        self.queue
            .iter()
            .map(|(seq, (_, item))| (*seq, item.clone()))
            .collect::<Vec<_>>()
    }

    pub fn flush_old(&mut self, threshold: u64) -> Vec<Item> {
        let old = self
            .queue
            .iter()
            .filter(|(_, (time, _))| (*time + threshold) < current_epoch())
            .map(|(_, (_, item))| item.clone())
            .collect::<Vec<_>>();
        self.queue
            .retain(|_, (time, _)| (*time + threshold) > current_epoch());
        old
    }
}

impl<Item> NetQueue<Item> for RecoveryQueue<Item> {
    type KeyId = u32;
    type Error = ();

    fn insert(&mut self, item: Item) -> Result<Self::KeyId, NetQueueError<Self::Error>> {
        let index = self.queue.len() as u32;
        self.queue.insert(index, (current_epoch(), item));
        Ok(index)
    }

    fn remove(&mut self, key: Self::KeyId) -> Result<Item, NetQueueError<Self::Error>> {
        if let Some((_, item)) = self.queue.remove(&key) {
            Ok(item)
        } else {
            Err(NetQueueError::ItemDeletionFail)
        }
    }

    fn get(&mut self, key: Self::KeyId) -> Result<&Item, NetQueueError<Self::Error>> {
        if let Some((_, item)) = self.queue.get(&key) {
            Ok(item)
        } else {
            Err(NetQueueError::ItemDeletionFail)
        }
    }

    fn flush(&mut self) -> Result<Vec<Item>, NetQueueError<Self::Error>> {
        let mut items = Vec::new();
        for (_, (_, item)) in self.queue.drain() {
            items.push(item);
        }
        Ok(items)
    }
}
