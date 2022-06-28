use tracing::info;

pub mod docker;
pub mod system;

pub async fn initalize_docker() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing Docker...");
    docker::create().await;
    docker::configure_docker().await;

    Ok(())
}
