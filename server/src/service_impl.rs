use log::info;
use rpc::services::RPCService;
use tarpc::context;

#[derive(Clone)]
pub struct RPCServiceImpl {}

#[tarpc::server]
impl RPCService for RPCServiceImpl {
    async fn ping(self, _: context::Context) -> Result<String, String> {
        info!("Ping Called.. responding with Pong!");
        Ok("Pong".into())
    }
    async fn echo(self, _: context::Context, value: String) -> Result<String, String> {
        info!("Echo Called.. responding with {}!", value);
        Ok(value)
    }
}
