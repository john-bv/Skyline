use relative_path::RelativePath;

mod net;
mod peer;
mod config;
pub(crate) mod utils;

#[tokio::main]
async fn main() {
    // boot strap env
    println!("Launching Skyline Server...");
    if let Err(_) = dotenv::dotenv() {
        println!("No .env file found, attempting to load cwd path.");

        // attempt to load the .env resource
        let locale = std::env::current_exe().unwrap();
        let rel = RelativePath::new(".env");
        if let Err(_) = dotenv::from_path(rel.to_path(&locale.parent().unwrap())) {
            println!(
                "Error: .env not found in {}, exiting...",
                rel.to_path(&locale).to_str().unwrap()
            );
            // exit
            std::process::exit(1);
        }
    }

    if cfg!(feature = "tcp") {}
}
