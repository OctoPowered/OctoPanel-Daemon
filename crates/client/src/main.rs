use colored::Colorize;
use server::ServerOptions;
use tracing::info;

mod constants;
mod dockerhandle;
mod logger;
mod server;
mod systemhandle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    println!("{}", constants::LOGO.bright_red());
    println!("{}", constants::SUBTEXT.bright_blue());

    info!("Thank you for using OctoDaemon! May your server live forever 🚀");

    environment::initalize_docker().await?;
    info!("Docker has been initialized.");

    let server_options = ServerOptions {
        ..Default::default()
    };

    server::create(server_options).await?;
    Ok(())
}
