use relative_path::RelativePath;
use skyline::net;
use skyline::{log_debug, log_error, log_info, log_notice, log_success, log_warn};

mod channel;
mod config;
mod peer;
mod server;
mod util;

#[tokio::main]
async fn main() {
    use colored::*;
    let (config, _) = match bootstrap() {
        Ok((config, verbosity)) => (config, verbosity),
        Err(e) => {
            log_error!("Failed to boot: {}", e);
            return;
        }
    };

    // initialize the server
    let mut server = match server::Server::new(&config).await {
        Ok(v) => v,
        Err(e) => {
            log_debug!("Error: {}", e);
            log_error!("A critcal error prevented skyline from initializing the server.");

            if std::env::var("SKYLINE_DEBUG").unwrap_or("0".to_string()) == "0" {
                log_error!("Run with debug mode enabled to see more information.");
            }

            std::process::exit(1);
        }
    };

    let closer = server.close.clone();

    if let Err(_) = ctrlc::set_handler(move || {
        closer.as_ref().notify_waiters();
    }) {
        log_error!("Failed to set SIGTERM handler");
        return;
    }

    match server.bind().await {
        Ok(_) => {}
        Err(e) => {
            log_error!("Failed to bind: {}", e);
            return;
        }
    }

    // start the server
    // this is now happening concurrently
    match server.start().await {
        Ok(_) => {}
        Err(e) => {
            log_error!("Failed to start: {}", e);
            return;
        }
    }

    // at this point the server is running
    // wait until the server is closed before continuing
    server.close.notified().await;
    log_success!("Stopped.");

    ()
}

/// Bootstraps the environment
/// Loads configs, sets up the database, etc.
fn bootstrap() -> std::io::Result<(self::config::Config, u8)> {
    // boot strap env
    use colored::*;

    // load config
    // try loading the config
    // get the config from the cwd path ./config.yaml
    let locale = match std::env::current_dir() {
        Ok(locale) => locale,
        Err(e) => {
            log_debug!("Could not load current dir: {}", e);
            log_error!("Skyline could not be loaded because of an OS error.");
            return Err(e);
        }
    };

    // verify the env file exists
    if !locale.join(".env.example").exists() {
        log_debug!(
            "Could not find .env.example in {}, creating it...",
            locale.to_str().unwrap()
        );
        match std::fs::File::create(RelativePath::new(".env.example").to_path(&locale)) {
            Ok(_) => {
                log_debug!("Successfully created .env.example");
                log_debug!("Storing random secrets in .env.example");
            }
            Err(e) => {
                log_error!("Failed to create .env.example. Please fix this issue and retry...\n -> Reason: {}", e);
                std::process::exit(1);
            }
        }
    }

    if let Err(_) = dotenv::dotenv() {
        // attempt to load the .env resource
        let locale = std::env::current_exe().unwrap();
        let rel = RelativePath::new(".env");
        if let Err(_) = dotenv::from_path(rel.to_path(&locale.parent().unwrap())) {
            log_debug!(
                "Error: .env not found in {}, random secrets will be used",
                rel.to_path(&locale).to_str().unwrap()
            );
            log_warn!(
                "No .env file found, please modify the .env.example file and rename it to .env"
            );
            std::process::exit(1);
        }
    }

    // Check the config to see if it exists
    if !locale.join("config.yaml").exists() {
        log_debug!("Could not find config.yaml in {}", locale.to_str().unwrap());
    }

    let default_config = include_str!("../resources/config.yaml");
    // log_debug!("Default config: {}", default_config);

    let user_config = match std::fs::read_to_string(
        RelativePath::new("config.yaml").to_path(&locale),
    ) {
        Ok(v) => {
            // check if the config is the same as the default config
            if v == default_config {
                log_warn!("You are using the default config, please edit the config.yaml file! Using the default config is not recommended as sykline will not persist!");
            }

            // attempt to parse the config
            match serde_yaml::from_str::<config::Config>(&v) {
                Ok(v) => {
                    log_debug!("Successfully loaded config.yaml");
                    v
                }
                Err(e) => {
                    log_error!("Failed to parse config.yaml: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            // attempt to create the config file
            let rel = RelativePath::new("config.yaml");
            if let Err(_) = std::fs::File::create(rel.to_path(&locale)) {
                log_error!(
                    "Error: Could not create config.yaml in {}",
                    rel.to_path(&locale).to_str().unwrap()
                );
                log_debug!("last error: {}", std::io::Error::last_os_error());
                std::process::exit(1);
            }

            // make the file
            match std::fs::write(rel.to_path(&locale), default_config) {
                Ok(_) => {
                    log_debug!("Should've wrote: {}", default_config);
                    log_info!("Successfully created config.yaml");
                }
                Err(e) => {
                    log_error!("Failed to create config.yaml. Please fix this issue and retry...\n -> Reason: {}", e);
                    std::process::exit(1);
                }
            }

            log_warn!("Please edit the config.yaml file to your liking and restart the server.");
            std::process::exit(0);
        }
    };

    return Ok((user_config, 0));
}
