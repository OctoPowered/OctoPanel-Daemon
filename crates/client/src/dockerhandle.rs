use std::{pin::Pin, time::Duration};

use environment::docker::containers;
use remote::rpc_docker::{
    docker_statistics_server::DockerStatistics, ContainerResourceStatsRequest,
    ContainerResourceStatsRequestStream, ContainerResourceStatsResponse, ContainerStatRequest,
    ContainerStatResponse,
};
use stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tokio_stream::{self as stream, StreamExt};
use tonic::{codegen::futures_core::Stream, Request, Response, Status};
use tracing::{debug, info};

use crate::server::EchoResult;

#[derive(Debug, Default)]
pub struct DockerRpcService {}

type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<ContainerResourceStatsResponse, Status>> + Send>>;

#[tonic::async_trait]
impl DockerStatistics for DockerRpcService {
    type StreamContainerResourceStatsStream = ResponseStream;
    async fn get_container_stat(
        &self,
        req: Request<ContainerStatRequest>,
    ) -> Result<Response<ContainerStatResponse>, Status> {
        info!("Got a request: {:?}", req);
        let container_id = req.get_ref().container_id.as_str();
        let container_stats = containers::get_container_stats(container_id).await;
        Ok(Response::new(container_stats.unwrap()))
    }
    async fn get_container_resource_stats(
        &self,
        req: Request<ContainerResourceStatsRequest>,
    ) -> Result<Response<ContainerResourceStatsResponse>, Status> {
        info!("Got a request: {:?}", req);
        let container_id = req.get_ref().container_id.as_str();
        let container_stats = containers::get_container_resource_stats(container_id).await;
        Ok(Response::new(container_stats.unwrap()))
    }
    async fn stream_container_resource_stats(
        &self,
        req: Request<ContainerResourceStatsRequestStream>,
    ) -> EchoResult<Self::StreamContainerResourceStatsStream> {
        info!("Got container resource stream request");
        info!("Recieved client connection from {:?}", req.remote_addr());
        let container_id = req.get_ref().container_id.as_str();

        let repeat = std::iter::repeat(
            containers::get_container_resource_stats(container_id)
                .await
                .unwrap(),
        );

        let mut interval = req.get_ref().interval;
        if interval <= 0 {
            interval = 10
        };

        let mut stream = Box::pin(
            stream::iter(repeat).throttle(Duration::from_secs(interval.try_into().unwrap())),
        );

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(send) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(send)).await {
                    Ok(_) => {
                        debug!(
                            "Sending container resource stats to client {:?}",
                            req.remote_addr()
                        );
                    }
                    Err(_item) => {
                        println!("error");
                        break;
                    }
                }
            }
            info!("All clients disconnected.");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::StreamContainerResourceStatsStream
        ))
    }
}
