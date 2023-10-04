pub mod conn;

use std::sync::Arc;

use tokio::sync::Notify;

use super::ListenerState;

pub struct TcpListener {
    state: ListenerState,
    listener: tokio::net::TcpListener,
    notifier: Arc<Notify>,
}

impl TcpListener {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(Self {
            state: ListenerState::Ready,
            listener,
            notifier: Arc::new(Notify::new()),
        })
    }

    pub async fn accept(&mut self) -> Result<tokio::net::TcpStream, Box<dyn std::error::Error>> {
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }

    pub fn state(&self) -> ListenerState {
        self.state
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.notifier.notify_waiters();
        self.state = ListenerState::Closed;
        Ok(())
    }
}