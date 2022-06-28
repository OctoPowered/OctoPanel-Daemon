use bollard::{models::Ipam, network::CreateNetworkOptions, Docker};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref DOCKER_INSTANCE: Mutex<Option<Docker>> = Mutex::new(Option::None);
}

/// Get a Docker instance.
/// This is a lazy static instance of a Docker client.
/// It is initialized on first call.
/// It is safe to call this function from multiple threads.
pub fn create() {
    let mut instance = DOCKER_INSTANCE
        .lock()
        .expect("Could not lock DOCKER_INSTANCE");

    if instance.is_none() {
        let docker = Docker::connect_with_local_defaults().expect("Could not connect to Docker");
        *instance = Some(docker);
    }
}

pub async fn configure_docker() {
    let mut instance = DOCKER_INSTANCE
        .lock()
        .expect("Could not lock DOCKER_INSTANCE");

    if instance.is_some() {
        let docker = instance.as_mut().unwrap();

        create_network("bridge", docker).await;
    }
}

async fn create_network(network_driver: &str, client: &Docker) {
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

    client
        .create_network(network_config)
        .await
        .expect("Could not create network");
}
