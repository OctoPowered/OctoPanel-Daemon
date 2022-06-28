use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use colored::Colorize;
use remote::{
    rpc_docker::docker_statistics_server::DockerStatisticsServer,
    system_statistics::system_transmitter_server::SystemTransmitterServer,
};
use tonic::transport::Server;
use tracing::info;

use crate::{dockerhandle::DockerRpcService, systemhandle::SystemService};

pub struct ServerOptions {
    pub address: IpAddr,
    pub port: u16,
}

impl Default for ServerOptions {
    fn default() -> Self {
        ServerOptions {
            address: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            port: 50051,
        }
    }
}

pub async fn create(options: ServerOptions) -> Result<(), tonic::transport::Error> {
    let addr = match options.address {
        IpAddr::V4(v4) => SocketAddr::new(IpAddr::V4(v4), options.port),
        IpAddr::V6(v6) => SocketAddr::new(IpAddr::V6(v6), options.port),
    };

    let service_system = SystemService::default();
    let docker_system = DockerRpcService::default();

    info!("Creating GRPC server...");

    info!(message = "Running gRPC server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("gRPC"))
        .add_service(SystemTransmitterServer::new(service_system))
        .add_service(DockerStatisticsServer::new(docker_system))
        .serve(addr)
        .await
}
