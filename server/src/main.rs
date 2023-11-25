use std::time::Duration;

use relative_path::RelativePath;

use crate::config::NetworkMode;

mod config;
mod net;
mod peer;
pub(crate) mod utils;

macro_rules! log_error {
    ($($msg:expr),*) => {
        println!("{}{}{}","[Skyline/ERROR] ".red(), "".clear(), format!($($msg),*).red());
    };
}

macro_rules! log_warn {
    ($($msg:expr),*) => {
        println!("{}{}{}","[Skyline/WARN] ".yellow(), "".clear(), format!($($msg),*).yellow());
    };
}

macro_rules! log_info {
    ($($msg:expr),*) => {
        println!("{}{}{}","[Skyline/INFO] ".white(), "".clear(), format!($($msg),*).white());
    };
}

macro_rules! log_notice {
    ($($msg:expr),*) => {
        println!("{}{}","[Skyline/NOTICE] ".blue(), format!($($msg),*).blue());
    };
}

macro_rules! log_debug {
    ($($msg:expr),*) => {
        println!("{}{}","[Skyline/DEBUG] ".truecolor(53, 53, 53), format!($($msg),*).truecolor(53, 53, 53));
    };
}

#[tokio::main]
async fn main() {
    use colored::*;
    let (config, verbosity) = match bootstrap() {
        Ok((config, verbosity)) => (config, verbosity),
        Err(e) => {
            log_error!("Failed to boot: {}", e);
            return;
        }
    };

    if config.network.mode == NetworkMode::Tcp {
        log_info!("Starting TCP server on port {}", config.port);
    }

    ()
}

/// Bootstraps the environment
/// Loads configs, sets up the database, etc.
fn bootstrap() -> std::io::Result<(self::config::Config, u8)> {
    // boot strap env
    use colored::*;

    if let Err(_) = dotenv::dotenv() {
        // attempt to load the .env resource
        let locale = std::env::current_exe().unwrap();
        let rel = RelativePath::new(".env");
        if let Err(_) = dotenv::from_path(rel.to_path(&locale.parent().unwrap())) {
            log_error!(
                "Error: .env not found in {}, exiting...",
                rel.to_path(&locale).to_str().unwrap()
            );
            // exit
            // std::process::exit(1);
        }

        log_warn!("Default settings will be used regadless of .env file presence.");

        // try loading the config
        // get the config from the cwd path ./config.yaml
        let locale = std::env::current_dir();
    }

    return Ok((Config::new(), 0));
}
