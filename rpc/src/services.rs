#[tarpc::service]
#[async_trait::async_trait]
pub trait RPCService {
    async fn ping() -> Result<String, String>;
    async fn echo(value: String) -> Result<String, String>;
}