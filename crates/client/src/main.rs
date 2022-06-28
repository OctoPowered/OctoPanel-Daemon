use log::info;
use server::ServerOptions;

mod constants;
mod dockerhandle;
mod logging;
mod server;
mod systemhandle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    environment::initalize_docker().await?;
    info!("Docker has been initialized.");

    let server_options = ServerOptions {
        ..Default::default()
    };

    server::create(server_options).await?;
    Ok(())
}
