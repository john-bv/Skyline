/// A Client is the primary way to interact with a server.
pub mod client;
pub mod conn;

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;

use super::{ListenerState, NetworkInterface};

pub struct TcpListener {
    state: ListenerState,
    listener: tokio::net::TcpListener,
    notifier: Arc<Notify>,
}

impl TcpListener {
    pub async fn init(addr: &str) -> std::io::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(Self {
            state: ListenerState::Ready,
            listener,
            notifier: Arc::new(Notify::new()),
        })
    }

    async fn internal_accept(&mut self) -> std::io::Result<tokio::net::TcpStream> {
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }

    pub fn state(&self) -> ListenerState {
        self.state
    }

    fn internal_close(&mut self) -> std::io::Result<()> {
        self.notifier.notify_waiters();
        self.state = ListenerState::Closed;
        Ok(())
    }
}

#[async_trait]
impl NetworkInterface for TcpListener {
    async fn new(addr: &str) -> std::io::Result<Self> {
        Self::init(addr).await
    }

    async fn bind(&mut self) -> std::io::Result<()> {
        self.state = ListenerState::Running;
        Ok(())
    }

    async fn accept(&mut self) -> std::io::Result<Box<dyn super::ConnAdapter>> {
        let stream = self.internal_accept().await?;
        let conn = conn::Conn::new(stream);
        Ok(Box::new(conn))
    }

    async fn close(&mut self) -> std::io::Result<()> {
        self.internal_close()
    }

    fn get_name(&self) -> &str {
        "tcp"
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        self.internal_close().unwrap();
    }
}
