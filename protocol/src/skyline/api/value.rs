use super::types::{Null, SkylineHashMap};
use binary_util::BinaryIo;
use std::{any, fmt::Debug};

/// Mainly for external use for the api.
/// This is used to identify the type of value.
#[derive(Debug, Clone, BinaryIo, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueIds {
    String = 0,
    Number = 1,
    Integer = 2,
    Boolean = 3,
    Null = 4,
    List = 5,
    Date = 6,
    Map = 7,
}

/// Value's are used to send data between the client and the server.
/// Think of this as a JSON value.
///
/// If a channel has an api enabled VIA the `Channel::has_api` field,
/// the client will use these type ids to determine what type of value
/// is being sent. Keep in mind that the value ID is known by both the client
/// and the server.
#[derive(Clone, Debug, PartialEq, BinaryIo)]
#[repr(u8)]
pub enum Value {
    /// A literal string value.
    /// { test: String }
    String(String) = 0,
    /// any number value.
    Number(f64),
    /// If explicit, an integer value.
    /// { test: i64 }
    Integer(i64),
    /// A boolean value.
    /// { test: bool }
    Boolean(bool),
    /// A null value.
    /// { test: () }
    Null(Null),
    /// A list of values.
    /// { test: Vec<Value> }
    List(Vec<Value>),
    /// A date value. (IN EPOCH)
    /// { test: u64 }
    Date(u64),
    /// A map of values.
    /// { test: HashMap<String, Value> }
    HashMap(SkylineHashMap),
}

impl Value {
    /// Returns the type id of the value.
    pub fn get_type(&self) -> ValueIds {
        match self {
            Value::String(_) => ValueIds::String,
            Value::Number(_) => ValueIds::Number,
            Value::Integer(_) => ValueIds::Integer,
            Value::Boolean(_) => ValueIds::Boolean,
            Value::Null(_) => ValueIds::Null,
            Value::List(_) => ValueIds::List,
            Value::Date(_) => ValueIds::Date,
            Value::HashMap(_) => ValueIds::Map,
        }
    }

    /// Just like get_type except it gets the inner value instead of a reference.
    pub fn inner(&self) -> &dyn any::Any {
        match self {
            Value::String(v) => v,
            Value::Number(v) => v,
            Value::Integer(v) => v,
            Value::Boolean(v) => v,
            Value::Null(v) => v,
            Value::List(v) => v,
            Value::Date(v) => v,
            Value::HashMap(v) => v,
        }
    }
}
