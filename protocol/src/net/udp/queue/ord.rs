use std::collections::BTreeMap;

/// An ordered queue is used to Index incoming packets over a channel.
#[derive(Debug, Clone)]
pub struct OrdQueue<Item: Clone + std::fmt::Debug> {
    pub queue: BTreeMap<u32, Item>,
    pub window: (u32, u32),
}

impl<Item> OrdQueue<Item>
where
    Item: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            queue: BTreeMap::new(),
            window: (0, 0),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.window.0 = self.window.0.wrapping_add(1);
        return self.window.0;
    }

    pub fn insert(&mut self, index: u32, item: Item) -> std::io::Result<()> {
        if index < self.window.0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Index is out of bounds",
            ));
        }
        if self.queue.contains_key(&index) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Index already exists",
            ));
        }

        if index >= self.window.1 {
            self.window.1 = index.wrapping_add(1);
        }

        self.queue.insert(index, item);
        return Ok(());
    }

    pub fn insert_abs(&mut self, index: u32, item: Item) -> std::io::Result<()> {
        if index >= self.window.1 {
            self.window.1 = index.wrapping_add(1);
        }

        self.queue.insert(index, item);
        return Ok(());
    }

    pub fn missing(&self) -> Vec<u32> {
        (self.window.0..self.window.1)
            .filter(|x| !self.queue.contains_key(x))
            .collect()
    }

    pub fn flush(&mut self) -> Vec<Item> {
        (self.window.0..self.window.1)
            .filter_map(|x| {
                if let Some(item) = self.queue.remove(&x) {
                    return Some(item);
                }
                return None;
            })
            .collect()
    }
}
