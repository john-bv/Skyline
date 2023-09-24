use std::collections::HashMap;

use crate::util::current_epoch;


pub struct Window {
    window: (u32, u32),
    size: u32,
    recv: HashMap<u32, u64>
}

impl Window {
    pub fn new() -> Self {
        Self {
            window: (0, 2048),
            size: 2048,
            recv: HashMap::new()
        }
    }

    /// Insert a sequence number into the window.
    /// Returns true if the sequence number is in the window.
    pub fn insert(&mut self, seq: u32) -> bool {
        if seq < self.window.0 || seq > self.window.1 {
            return false;
        }

        self.recv.insert(seq, current_epoch());

        if seq == self.window.0 {
            self.adjust();
        }

        return true;
    }

    pub fn adjust(&mut self) {
        while self.recv.contains_key(&self.window.0) {
            self.recv.remove(&self.window.0);
            self.window.0 = self.window.0.wrapping_add(1);
            self.window.1 = self.window.1.wrapping_add(1);
        }

        let current = self.window.1.wrapping_sub(self.window.0);
        if current < self.size {
            self.window.1 = self.window.0.wrapping_add(self.size);
        } else if current > self.size {
            self.window.0 = self.window.1.wrapping_sub(self.size);
        }
    }

    pub fn missing(&self) -> Vec<u32> {
        (self.window.0..self.window.1)
            .into_iter()
            .filter(|seq| !self.recv.contains_key(seq))
            .map(|seq| seq)
            .collect()
    }

    pub fn window(&self) -> std::ops::Range<u32> {
        self.window.0..self.window.1
    }

    pub fn cleanup(&mut self) {
        // clean up old packets.
        self.recv
            .retain(|seq, _| *seq > self.window.0 && *seq < self.window.1);
    }

    /// Purge packets that are older than 60 seconds.
    pub fn purge_old(&mut self) {
        self.cleanup();

        // store now so it's constant for the loop.
        let now = current_epoch();
        self.recv.retain(|_, v| *v > (now + 60));
    }
}