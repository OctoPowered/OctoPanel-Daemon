use std::collections::HashMap;

use bollard::{
    container::{Config, CreateContainerOptions, StatsOptions},
    models::ContainerStateStatusEnum,
};
use futures::StreamExt;
use remote::rpc_docker::{ContainerResourceStatsResponse, ContainerStatResponse, ResourceStats};
use uuid::Uuid;

use super::{DockerResult, DOCKER_INSTANCE};

pub async fn get_container_stats(container_id: &str) -> DockerResult<ContainerStatResponse> {
    let instance = DOCKER_INSTANCE.lock().await;

    let docker = instance.as_ref().unwrap();

    if let Ok(response) = docker.inspect_container(container_id, None).await {
        Ok(ContainerStatResponse {
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

pub async fn get_container_resource_stats(
    container_id: &str,
) -> DockerResult<ContainerResourceStatsResponse> {
    let instance = DOCKER_INSTANCE.lock().await;

    let docker = instance.as_ref().unwrap();

    let opts = StatsOptions {
        ..Default::default()
    };

    let stream = &mut docker.stats(container_id, Some(opts)).take(1);

    let mut container_stats: Option<ContainerResourceStatsResponse> = None;

    while let Some(Ok(stat)) = stream.next().await {
        // TODO: fix network statistics since they only return 0
        let net_rx = if stat.network.is_some() {
            stat.network.unwrap().rx_bytes
        } else {
            0
        };
        let net_tx = if stat.network.is_some() {
            stat.network.unwrap().tx_bytes
        } else {
            0
        };
        container_stats = Some(ContainerResourceStatsResponse {
            resources: Some(ResourceStats {
                memory_usage: stat.memory_stats.usage.unwrap_or(0),
                memory_limit: stat.memory_stats.limit.unwrap_or(0),
                cpu_usage: stat.cpu_stats.cpu_usage.total_usage,
                network_rx: net_rx,
                network_tx: net_tx,
            }),
        });
        break;
    }

    if container_stats.is_some() {
        Ok(container_stats.unwrap())
    } else {
        return Err("Could not get container stats".into());
    }
}

pub struct CreateContainer {
    name: String,
    image: String,
    uuid: Option<Uuid>,
    env: Option<Vec<String>>,
}

pub async fn create_container(opts: CreateContainer) {
    let instance = DOCKER_INSTANCE.lock().await;

    let docker = instance.as_ref().unwrap();

    let options = CreateContainerOptions { name: opts.name };

    let id = opts.uuid.unwrap_or(Uuid::new_v4()).to_string();

    let mut labels = HashMap::new();
    labels.insert("octo-id".to_string(), id);

    let config = Config {
        image: Some(opts.image),
        attach_stderr: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        labels: Some(labels),
        env: opts.env,
        ..Default::default()
    };

    docker.create_container(Some(options), config).await;
}
