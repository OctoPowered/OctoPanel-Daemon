use bollard::{
    models::{ContainerStateStatusEnum, Ipam, NetworkCreateResponse},
    network::CreateNetworkOptions,
    Docker,
};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use remote::rpc_docker::{GetContainerStatRequest, GetContainerStatResponse};
use std::collections::HashMap;
use tracing::{debug, info};

type DockerResult<T> = Result<T, Box<dyn std::error::Error>>;
type BollardResult<T> = Result<T, bollard::errors::Error>;

lazy_static! {
    static ref DOCKER_INSTANCE: Mutex<Option<Docker>> = Mutex::new(Option::None);
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
        info!("Already created Docker's OctoNet network. Skipping...");
        debug!("{:?}", err);
    } else {
        info!("Created Docker's OctoNet network.");
    }
}

pub async fn get_container_stats(container_id: &str) -> DockerResult<GetContainerStatResponse> {
    let instance = DOCKER_INSTANCE.lock().await;

    let docker = instance.as_ref().unwrap();

    if let Ok(response) = docker.inspect_container(container_id, None).await {
        Ok(GetContainerStatResponse {
            container_id: response.id.unwrap_or("Unknown".to_string()),
            name: response.name.unwrap_or("Unknown".to_string()),
            image: response.image.unwrap_or("Unknown".to_string()),
            status: response
                .state
                .unwrap()
                .status
                .unwrap_or(ContainerStateStatusEnum::EMPTY)
                .to_string(),
            created: response.created.unwrap_or("Unknown".to_string()),
        })
    } else {
        return Err("Could not get container stats".into());
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
