use bollard::models::ContainerStateStatusEnum;
use remote::rpc_docker::GetContainerStatResponse;

use super::{DockerResult, DOCKER_INSTANCE};

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
