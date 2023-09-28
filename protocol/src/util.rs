use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn current_epoch_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}


#[derive(Debug, Clone)]
pub struct SafeGenerator<T> {
    pub(crate) sequence: T,
}

impl<T> SafeGenerator<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            sequence: T::default(),
        }
    }
}

macro_rules! impl_gen {
    ($n: ty) => {
        impl SafeGenerator<$n> {
            pub fn next(&mut self) -> $n {
                self.sequence = self.sequence.wrapping_add(1);
                return self.sequence;
            }

            pub fn get(&self) -> $n {
                self.sequence
            }
        }
    };
}

impl_gen!(u8);
impl_gen!(u16);
impl_gen!(u32);
impl_gen!(u64);
impl_gen!(u128);
impl_gen!(usize);