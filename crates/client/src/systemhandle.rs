use remote::system_statistics::{
    system_transmitter_server::SystemTransmitter, SystemStatsReponse, SystemStatsRequest,
};
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Debug, Default)]
pub struct SystemService {}

#[tonic::async_trait]
impl SystemTransmitter for SystemService {
    async fn get_system_stats(
        &self,
        req: Request<SystemStatsRequest>,
    ) -> Result<Response<SystemStatsReponse>, Status> {
        info!("Got a request: {:?}", req);

        Ok(Response::new(environment::system::get_system_stats()))
    }
}
