#![allow(dead_code)]
use std::net::SocketAddr;

/// This is a helper enum that allows you to pass in a `SocketAddr` or a `&str` to the `Listener::bind` function.
/// This is useful for when you want to bind to a specific address, but you don't want to parse it yourself.
///
/// This Trait will successfully parse the following:
/// - `SocketAddr::new("127.0.0.1:19132")`
/// - `"127.0.0.1:19132"`
/// - `String::from("127.0.0.1:19132")`
pub enum PossiblySocketAddr<'a> {
    SocketAddr(SocketAddr),
    Str(&'a str),
    String(String),
    ActuallyNot,
}

impl PossiblySocketAddr<'_> {
    pub fn to_socket_addr(self) -> Option<SocketAddr> {
        match self {
            PossiblySocketAddr::SocketAddr(addr) => Some(addr),
            PossiblySocketAddr::Str(addr) => {
                // we need to parse it
                Some(addr.parse::<SocketAddr>().unwrap())
            }
            PossiblySocketAddr::String(addr) => {
                // same as above, except less elegant >_<
                Some(addr.clone().as_str().parse::<SocketAddr>().unwrap())
            }
            _ => None,
        }
    }
}

impl From<&str> for PossiblySocketAddr<'_> {
    fn from(s: &str) -> Self {
        PossiblySocketAddr::String(s.to_string())
    }
}

impl From<String> for PossiblySocketAddr<'_> {
    fn from(s: String) -> Self {
        PossiblySocketAddr::String(s)
    }
}

impl From<SocketAddr> for PossiblySocketAddr<'_> {
    fn from(s: SocketAddr) -> Self {
        PossiblySocketAddr::SocketAddr(s)
    }
}

impl std::fmt::Display for PossiblySocketAddr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PossiblySocketAddr::SocketAddr(addr) => write!(f, "{}", addr),
            PossiblySocketAddr::Str(addr) => write!(f, "{}", addr),
            PossiblySocketAddr::String(addr) => write!(f, "{}", addr),
            PossiblySocketAddr::ActuallyNot => write!(f, "Not a valid address!"),
        }
    }
}