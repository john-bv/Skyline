pub mod ord;
pub mod recovery;
pub mod recv;
pub mod send;
pub mod split;
pub mod window;

#[derive(Debug, Clone)]
pub enum NetQueueError<E> {
    /// The insertion failed for any given reason.
    InvalidInsertion,
    /// The insertion failed and the reason is known.
    InvalidInsertionKnown(String),
    /// The `Item` failed to be removed from the queue.
    ItemDeletionFail,
    /// The `Item` is invalid and can not be retrieved.
    InvalidItem,
    /// The queue is empty.
    EmptyQueue,
    /// The error is a custom error.
    Other(E),
}

pub trait NetQueue<Item> {
    /// The `Item` of the queue.
    // type Item = V;

    /// The "key" that each `Item` is stored under
    /// (used for removal)
    type KeyId;

    /// A custom error specifier for NetQueueError
    type Error;

    /// Inserts `Item` into the queue, given the conditions are fulfilled.
    fn insert(&mut self, item: Item) -> Result<Self::KeyId, NetQueueError<Self::Error>>;

    /// Remove an `Item` from the queue by providing an instance of `Self::KeyId`
    fn remove(&mut self, key: Self::KeyId) -> Result<Item, NetQueueError<Self::Error>>;

    /// Retrieves an `Item` from the queue, by reference.
    fn get(&mut self, key: Self::KeyId) -> Result<&Item, NetQueueError<Self::Error>>;

    /// Clears the entire queue.
    fn flush(&mut self) -> Result<Vec<Item>, NetQueueError<Self::Error>>;
}
