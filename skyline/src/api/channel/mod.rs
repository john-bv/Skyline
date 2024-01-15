pub mod channel;

use std::{any::Any, cell::RefCell, collections::HashMap, hash::Hash, sync::Arc};

use self::channel::Channel;

pub const CX_FIXED: usize = 1048;

/// A channel Pool is a collection of channels.
/// You can think of it as Multiple channels on a single server.
///
/// You can digest a message inside a pool using the `digest` method.
pub struct ChannelPool {
    // cursed lmao
    pub channels: Arc<RefCell<HashMap<String, Box<Channel>>>>,
}

impl ChannelPool {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn digest(&mut self, message: Vec<u8>) {}

    pub fn add_channel(&mut self, channel: Box<Channel>) {
        let mut chans = self.channels.borrow_mut();
        chans.insert(channel.name.clone(), channel);
    }
}
