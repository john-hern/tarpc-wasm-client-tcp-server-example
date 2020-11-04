use log::info;
use rpc::services::PingService;
use tarpc::{context, server};

#[derive(Clone)]
pub struct PingServiceImpl {}
#[tarpc::server]
impl PingService for PingServiceImpl {
    async fn ping(self, _: context::Context) -> Result<String, String> {
        info!("Ping Called.. responding with Pong!");
        Ok("Pong".into())
    }
}
