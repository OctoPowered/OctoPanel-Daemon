use bollard::{
    models::{Ipam, NetworkCreateResponse},
    network::CreateNetworkOptions,
    Docker,
};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tracing::{debug, info};

pub type DockerResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type BollardResult<T> = Result<T, bollard::errors::Error>;

pub mod containers;

lazy_static! {
    pub static ref DOCKER_INSTANCE: Mutex<Option<Docker>> = Mutex::new(Option::None);
}

/// Get a Docker instance.
/// This is a lazy static instance of a Docker client.
/// It is initialized on first call.
/// It is safe to call this function from multiple threads.
pub async fn create() {
    let mut instance = DOCKER_INSTANCE.lock().await;

    if instance.is_none() {
        let docker = Docker::connect_with_local_defaults().expect("Could not connect to Docker");
        *instance = Some(docker);
    }
}

pub async fn configure_docker() {
    let instance = DOCKER_INSTANCE.lock().await;

    let docker = instance.as_ref().unwrap();

    info!("Configuring Docker...");
    if let Err(err) = create_network("bridge", &docker).await {
        match err {
            bollard::errors::Error::DockerResponseServerError {
                status_code,
                message,
            } => {
                info!("Already created Docker's OctoNet network. Skipping...");
                debug!("Status code: {:?}\n\tMessage: {:?}", status_code, message);
            }
            err => {
                panic!("Could not create Docker's OctoNet network Error: {:?}", err);
            }
        };
    } else {
        info!("Created Docker's OctoNet network.");
    }
}

async fn create_network(
    network_driver: &str,
    client: &Docker,
) -> BollardResult<NetworkCreateResponse> {
    let network_config = CreateNetworkOptions {
        driver: network_driver,
        name: "octonet",
        check_duplicate: true,
        internal: false,
        attachable: false,
        ingress: false,
        ipam: Ipam::default(),
        enable_ipv6: false,
        options: HashMap::new(),
        labels: HashMap::new(),
    };

    client.create_network(network_config).await
}
