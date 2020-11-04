


#[tarpc::service]
pub trait RPCService {
    async fn ping() -> Result<String, String>;
    async fn echo(value: String) -> Result<String, String>;
}
/*
#[derive(Clone)]
pub struct PingServiceImpl{

}
#[tarpc::server]
impl PingService for PingServiceImpl {
    async fn ping(self, _: context::Context) -> Result<String, String> {
        Ok("Pong".into())
    }
}
*/
