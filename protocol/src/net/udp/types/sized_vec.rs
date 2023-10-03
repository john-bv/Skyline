use binary_util::interfaces::{Reader, Writer};

#[derive(Debug, Clone)]
pub struct SizedVec<S, T> {
    pub len: S,
    pub data: Vec<T>,
}

/// S can only be u8, u16, u32, u64, u128
impl<S, T> Writer for SizedVec<S, T>
where
    S: Writer,
    T: Writer,
{
    fn write(&self, buf: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        self.len.write(buf)?;
        for data in &self.data {
            data.write(buf)?;
        }
        Ok(())
    }
}

impl<S, T> SizedVec<S, T>
where
    S: Clone,
{
    pub fn new(len: S) -> Self {
        Self {
            len,
            data: Vec::new(),
        }
    }

    pub fn with_capacity(len: S, capacity: usize) -> Self {
        Self {
            len,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, data: T) {
        self.data.push(data);
    }

    pub fn len(&self) -> S {
        self.len.clone()
    }
}

macro_rules! impl_sized_vec_reader {
    ($($prim: ty),*) => {
        $(
            impl<T> Reader<SizedVec<$prim, T>> for SizedVec<$prim, T>
            where
                T: Reader<T>
            {
                fn read(buf: &mut binary_util::ByteReader) -> Result<SizedVec<$prim, T>, std::io::Error> {
                    let len = <$prim>::read(buf)?;
                    let mut data = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        data.push(T::read(buf)?);
                    }
                    Ok(SizedVec {
                        len,
                        data
                    })
                }
            }

            // here we implement Into<Vec<T>> for SizedVec<$prim, T>
            // allows casting.

            impl<T> Into<Vec<T>> for SizedVec<$prim, T> {
                fn into(self) -> Vec<T> {
                    self.data
                }
            }

            impl<T> From<Vec<T>> for SizedVec<$prim, T> {
                fn from(data: Vec<T>) -> Self {
                    Self {
                        len: data.len() as $prim,
                        data
                    }
                }
            }

            impl<T, const N: usize> From<[T; N]> for SizedVec<$prim, T>
            where
                T: Clone {
                fn from(data: [T; N]) -> Self {
                    Self {
                        len: data.len() as $prim,
                        data: data.to_vec()
                    }
                }
            }

            impl<T> From<SizedVec<$prim, T>> for [T; 0] {
                fn from(_: SizedVec<$prim, T>) -> Self {
                    []
                }
            }

            // slice ref
            impl<T> From<&[T]> for SizedVec<$prim, T>
            where
                T: Clone {
                fn from(data: &[T]) -> Self {
                    Self {
                        len: data.len() as $prim,
                        data: data.to_vec()
                    }
                }
            }
        )*
    };
}

impl_sized_vec_reader! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128
}
