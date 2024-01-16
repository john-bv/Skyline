use std::sync::{Arc, Mutex};

use tokio::{sync::Notify, task::JoinHandle};
use tokio::sync::Mutex as TokioMutex;

use crate::{
    config::NetworkMode,
    log_debug, log_error, log_notice, log_success, log_warn,
    net::NetworkInterface,
    peer::{Peer, PeerManager},
};

use colored::*;

/// This is the main struct responsible for managing the server.
/// It will handle all the connections, and will be responsible for
/// managing the database.
///
/// The server will initialize depending on the network mode specified
/// in the config. IE, the TCP interface will be used if TCP is specified.
pub struct Server {
    mode: NetworkMode,
    pub close: Arc<Notify>,
    config: crate::config::Config,
    interface: Arc<TokioMutex<Box<dyn crate::net::NetworkInterface>>>,
    peer_manager: Arc<TokioMutex<PeerManager>>
}

impl Server {
    pub async fn new(config: &crate::config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        let bind_address = format!("0.0.0.0:{}", config.port);
        let mode = config.network.mode.clone();
        let close = Arc::new(Notify::new());
        Ok(Self {
            mode,
            close,
            config: config.clone(),
            interface: match mode {
                NetworkMode::Tcp => {
                    log_debug!("TCP mode selected, binding to {}", bind_address);
                    log_warn!("TCP mode selected by config file, with multiple clients (over 200) this may cause performance issues.");
                    Arc::new(TokioMutex::new(Box::new(crate::net::tcp::TcpListener::new(bind_address.as_str()).await?)))
                }
                // NetworkMode::Udp => Box::new(crate::net::udp::UdpListener::new(address)?),
                _ => {
                    log_error!(
                        "Unsupported network mode: {}, attempting to start anyway...",
                        mode
                    );
                    Arc::new(TokioMutex::new(Box::new(crate::net::NullInterface::new(bind_address.as_str()).await?)))
                }
            },
            peer_manager: Arc::new(TokioMutex::new(PeerManager::new()))
        })
    }

    pub async fn bind(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interface = self.interface.lock().await;
        if interface.get_name() == "null" {
            log_debug!("Refusing to bind: null interface");
            log_error!("Skyline ran into an error while binding to the interface.");
            log_error!("Please check your configuration and try again.");
            std::process::exit(1);
        }

        interface.bind().await?;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log_warn!("Skyline does not currently support database connections.");

        // database stuff above this...

        log_success!("Skyline is now listening on port {}.", self.config.port);

        let peer_manager = self.peer_manager.clone();
        let close_notifier = self.close.clone();

        // network recv clients
        let net_interface = self.interface.clone();
        tokio::task::spawn(async move {
            loop {
                let mut interface = net_interface.lock().await;
                tokio::select! {
                    _ = close_notifier.notified() => {
                        log_notice!("Closing...");
                        break;
                    }
                    conn = interface.accept() => {
                        match conn {
                            Ok(ref conn) => {
                                log_debug!("Accepted connection from {}", conn.lock().unwrap().get_addr());
                            }
                            Err(e) => {
                                log_debug!("Failed to accept connection: {}", e);
                                continue;
                            }
                        };
                        // create a new peer with this connection
                        let closer = close_notifier.clone();

                        let conn = conn.unwrap();
                        let peer = Box::new(Peer::init(conn, closer).await);

                        let manager = peer_manager.clone();
                        let mut manager = manager.lock().await;
                        manager.add_peer(peer);
                    }
                }
            }
        });
        Ok(())
    }

    /// This is sync cause we should be able to call it from anywhere.
    /// It will close the server, and will wait for all the tasks to finish.
    ///
    /// **THIS IS A BLOCKING CALL, MAIN THREAD WILL BE BLOCKED UNTIL ALL TASKS ARE FINISHED.**
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.close.notify_waiters();
        let handle = tokio::runtime::Handle::current();
        let interface = self.interface.clone();
        if let Err(_) = handle.block_on(async move {
            interface.lock().await.close().await
        }) {
            log_error!("Failed to close network interface.");
        }

        Ok(())
    }
}
