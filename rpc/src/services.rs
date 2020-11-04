


#[tarpc::service]
pub trait PingService {
    async fn ping() -> Result<String, String>;
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
