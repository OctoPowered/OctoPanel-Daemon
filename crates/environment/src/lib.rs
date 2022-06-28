pub mod docker;
pub mod system;

pub async fn initalize_docker() -> Result<(), Box<dyn std::error::Error>> {
    docker::create();
    docker::configure_docker().await;

    Ok(())
}
