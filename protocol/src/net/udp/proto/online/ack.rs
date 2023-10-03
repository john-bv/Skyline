use binary_util::BinaryIo;
pub trait Acknowledgeable {
    type NackItem;

    /// When an ac packet is recieved.
    /// We should ack the queue.
    fn ack(&mut self, _: Acknowledgement) {}

    fn nack(&mut self, _: Acknowledgement) -> Vec<Self::NackItem> {
        todo!("implement nack")
    }
}

#[derive(BinaryIo)]
#[repr(u8)]
pub enum AckVariant {
    /// Acknowledge a packet.
    Ack(Acknowledgement) = 1,
    /// Request acknowledgement for a packet.
    /// No Acknowledgement
    Nack(Acknowledgement) = 0,
}

#[derive(BinaryIo, Clone)]
pub struct Acknowledgement {
    /// A list of sequences we are missing.
    /// (In no particular order)
    pub seqs: Vec<u32>,
    /// If this is true,
    /// it is a list of acknowledgements to sent split packets.
    pub splits: Option<Vec<u32>>,
}

impl Acknowledgement {
    pub fn new() -> Self {
        Self {
            seqs: Vec::new(),
            splits: None,
        }
    }

    pub fn add(&mut self, seq: u32) {
        self.seqs.push(seq);
    }

    pub fn add_split(&mut self, seq: u32) {
        if self.splits.is_none() {
            self.splits = Some(Vec::new());
        }
        self.splits.as_mut().unwrap().push(seq);
    }

    pub fn to_nack(&self) -> AckVariant {
        AckVariant::Nack(self.clone())
    }

    pub fn to_ack(&self) -> AckVariant {
        AckVariant::Ack(self.clone())
    }
}
