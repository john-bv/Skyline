use binary_util::interfaces::{Reader, Writer};
use binary_util::BinaryIo;

use super::value::Value;

/// A null type that is used to represent a null value.
/// Has no data.
#[derive(Debug, Clone, BinaryIo, PartialEq, Eq)]
pub struct Null {}

#[derive(Debug, Clone, PartialEq)]
pub struct SkylineHashMap {
    map: Vec<(Value, Value)>,
}

impl SkylineHashMap {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    pub fn contains_key(&self, key: &Value) -> bool {
        self.map.iter().any(|(k, _)| k == key)
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.map.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &Value) -> Option<&mut Value> {
        self.map
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        if let Some(index) = self.map.iter().position(|(k, _)| k == key) {
            Some(self.map.remove(index).1)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        if self.contains_key(&key) {
            self.map
                .iter_mut()
                .find(|(k, _)| k == &key)
                .map(|(_, v)| *v = value);
            return;
        }
        self.map.push((key, value));
    }
}

impl Reader<SkylineHashMap> for SkylineHashMap {
    fn read(buf: &mut binary_util::ByteReader) -> Result<Self, std::io::Error> {
        // skyline encodes a hashmap as ELEMENTS,KEY,TYPE,VALUE,
        let mut map = Self::new();
        let amount = buf.read_var_u32()?;

        // read all the elements
        for _ in 0..amount {
            // THE KEY IS NOT ALWAYS A STRING,
            // IT CAN ONLY BE A STRING, NUMBER, OR BOOLEAN.
            // todo stop parsing as a value and parse as a string, number, or boolean
            // match buf.read_type::<Value>()? {
            //     Value::String(_) => {}
            //     Value::Number(_) => {}
            //     Value::Boolean(_) => {}
            //     _ => {
            //         return Err(std::io::Error::new(
            //             std::io::ErrorKind::InvalidData,
            //             "key is not a string, number, or boolean",
            //         ))
            //     }
            // }
            let key = buf.read_type::<Value>()?;
            let value = buf.read_type::<Value>()?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl Writer for SkylineHashMap {
    fn write(&self, buf: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        // skyline encodes a hashmap as ELEMENTS,KEY,TYPE,VALUE,
        buf.write_var_u32(self.map.len() as u32)?;

        // write all the elements
        for (key, value) in &self.map {
            buf.write_type(key)?;
            buf.write_type(value)?;
        }

        Ok(())
    }
}
