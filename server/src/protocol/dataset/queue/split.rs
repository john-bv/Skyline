use std::collections::HashMap;

use crate::protocol::net::{online::dataset::{DataSet, SplitInfo, DataBits}, MAX_PROTO_OVERHEAD};

/// A reliable split packet queue.
/// This struct will handle splitting packets into multiple packets, as well as reassembling them.
pub struct SplitQueue {
    /// This is the current frag id number.
    /// If for whatever reason, this overlaps with an existing frame, it will be replaced.
    id: u32,

    /// The current splits
    /// Hashmap represends the following values:
    /// (size, Vec<DataSet>)
    splits: HashMap<u16, (u32, Vec<DataSet>)>
}

impl SplitQueue {
    pub fn new() -> Self {
        Self {
            id: 0,
            splits: HashMap::new()
        }
    }

    /// This will add a buffer to the split queue and split it in the process, returning it's ID.
    pub fn add(&mut self, buffer: &[u8], mtu: u16) -> Result<u16, SplitQueueError> {
        self.id += self.id.wrapping_add(1);

        let id = self.id as u16;

        if self.splits.contains_key(&id) {
            self.splits.remove(&id);
        }

        if let Ok(splits) = Self::split(buffer, id, mtu) {
            self.splits.insert(id, (splits.len() as u32, splits));
            return Ok(id);
        }

        return Err(SplitQueueError::NotRequired);
    }

    /// This inserts a split into the split queue
    /// This will return a tuple of the size of the split, and the current index of the split.
    /// IE: (size, index)
    ///
    /// This is primarily used to digest split payloads coming in.
    pub fn insert(&mut self, set: DataSet) -> Result<(u32, u32), SplitQueueError> {
        if let Some(split_info) = set.split.as_ref() {
            // check if we have this split already, if we don't insert it.
            // if we do, process it.
            if let Some((size, splits)) = self.splits.get_mut(&split_info.id) {
                if split_info.index >= *size {
                    return Err(SplitQueueError::IndexOutOfBounds);
                }

                // check to see if we already have this split piece, if we do, it's invalid.
                if let Some(_) = splits
                    .iter()
                    .find(|s| s.split.as_ref().unwrap().index == split_info.index)
                {
                    return Err(SplitQueueError::Exists);
                } else {
                    // insert the split into the splits vec.
                    let ret = (*size, split_info.index);
                    splits.push(set);
                    return Ok(ret);
                }
            } else {
                // This is a new split, insert it.
                let (size, mut splits) = (split_info.total, Vec::<DataSet>::new());
                let (id, index) = (split_info.id, split_info.index);
                splits.push(set);

                self.splits.insert(id, (size, splits));
                return Ok((size, index));
            }
        }

        return Err(SplitQueueError::NotSplit);
    }

    /// Attempts to join all splits into a single buffer.
    /// This will fail if there are missing indices.
    pub fn join(&mut self, id: u16) -> Result<Vec<u8>, SplitQueueError> {
        if let Some((size, splits)) = self.splits.get_mut(&id) {
            if *size == splits.len() as u32 {
                // we have all the splits! Sort them.
                splits.sort_by(|a, b| {
                    a.split.as_ref().unwrap().index.cmp(&b.split.as_ref().unwrap().index)
                });

                let mut buf = Vec::<u8>::new();

                for split in splits.iter() {
                    buf.extend_from_slice(&split.payload);
                }

                self.splits.remove(&id);

                return Ok(buf);
            } else {
                return Err(SplitQueueError::MissingIndices);
            }
        }

        return Err(SplitQueueError::InvalidIndex);
    }

    pub fn split(buffer: &[u8], id: u16, mtu: u16) -> Result<Vec<DataSet>, SplitQueueError> {
        let max_mtu = mtu - MAX_PROTO_OVERHEAD;

        if buffer.len() > max_mtu.into() {
            let splits = buffer
                .chunks(max_mtu.into())
                .map(|c| c.to_vec())
                .collect::<Vec<Vec<u8>>>();
            let mut sets: Vec<DataSet> = Vec::new();
            let mut index: u32 = 0;

            for buf in splits.iter() {
                let set = DataSet {
                    flags: DataBits::Split,
                    seq: 0.into(),
                    split: Some(SplitInfo {
                        id,
                        total: splits.len() as u32,
                        index
                    }),
                    order: None,
                    payload: buf.clone()
                };

                sets.push(set);
                index += 1;
            }

            return Ok(sets);
        }

        return Err(SplitQueueError::NotRequired);
    }

    /// Utility fn to get a reference to a split id.
    pub fn get(&self, id: &u16) -> Result<&(u32, Vec<DataSet>), SplitQueueError> {
        if let Some(split) = self.splits.get(id) {
            return Ok(split);
        }

        return Err(SplitQueueError::InvalidIndex);
    }

    /// Utility fn to get a mutable reference to a split id.
    pub fn get_mut(&mut self, id: &u16) -> Result<&mut (u32, Vec<DataSet>), SplitQueueError> {
        if let Some(split) = self.splits.get_mut(id) {
            return Ok(split);
        }

        return Err(SplitQueueError::InvalidIndex);
    }

    pub fn remove(&mut self, id: &u16) -> Result<(u32, Vec<DataSet>), SplitQueueError> {
        if let Some(split) = self.splits.remove(id) {
            return Ok(split);
        }

        return Err(SplitQueueError::InvalidIndex);
    }

    pub fn clear(&mut self) {
        self.id = 0;
        self.splits.clear();
    }

    pub fn len(&self) -> usize {
        self.splits.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SplitQueueError {
    /// This error occurs when the split id being inserted has already been inserted.
    /// IE: A split with an id of 1 and index of 2, has already been inserted.
    Exists,
    /// This error occurs when the dataset is not split into pieces.
    /// Therefore no further processing can occur on this dataset.
    NotSplit,
    /// This error occurs when you try to split a buffer that is too small.
    /// And you do not need to split it.
    NotRequired,
    /// This error occurs when you try to get a split that does not exist.
    InvalidIndex,
    /// Occurs only when you try to `join` a split that is missing indices.
    /// This is not fatal, but should be handled properly because you can not process
    /// such splits reliably.
    MissingIndices,
    /// This error occurs when you try to process a split that has a index out of bounds.
    /// For example, the split size is 10, and you try to insert an index of 11.
    IndexOutOfBounds,
}