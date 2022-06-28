use environment::docker::containers;
use remote::rpc_docker::{
    docker_statistics_server::DockerStatistics, GetContainerStatRequest, GetContainerStatResponse,
};
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Debug, Default)]
pub struct DockerRpcService {}

#[tonic::async_trait]
impl DockerStatistics for DockerRpcService {
    async fn get_container_stat(
        &self,
        req: Request<GetContainerStatRequest>,
    ) -> Result<Response<GetContainerStatResponse>, Status> {
        info!("Got a request: {:?}", req);
        let container_id = req.get_ref().container_id.as_str();
        let container_stats = containers::get_container_stats(container_id).await;
        Ok(Response::new(container_stats.unwrap()))
    }
}
