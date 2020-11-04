use ::rpc::services;
use ::server::loggin::LogWriter;
use ::server::service_impl::PingServiceImpl;
use ::server::web::bind;
use futures_util::StreamExt;
use futures_util::*;
use log::info;
use serde::{Deserialize, Serialize};
use services::PingService;

use tarpc;


use tarpc::server::*;



use tokio::stream::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_writer = LogWriter::new(log::Level::Info);
    if let Ok(()) = log::set_boxed_logger(Box::new(log_writer)) {
        log::set_max_level(log::LevelFilter::Info)
    }
    info!("First Message");

    let server = build_server().await.expect("Failed to get server channel");
    let stream = server.map_ok(move |x| {
        info!("Mapping the client session");
        let channel = tarpc::server::BaseChannel::with_defaults(x);
        let server = PingServiceImpl {};
        info!("Spawning client channel");
        tokio::spawn(channel.respond_with(server.serve()).execute())
    });

    //TODO: Will likely need a way to kill the connection. Need to figure that out.
    let handle = tokio::spawn(stream.for_each(|_| async {}));
    handle.await.unwrap();
    Ok(())
}

async fn build_server<Item, SinkItem>(
) -> Option<impl Stream<Item = Result<impl tarpc::Transport<SinkItem, Item>, std::io::Error>>>
where
    Item: for<'de> Deserialize<'de> + Unpin,
    SinkItem: Serialize + Unpin,
{
    Some(
        bind(tokio_serde::formats::Json::<Item, SinkItem>::default)
            .await
            .unwrap(),
    )

    //self.server_handles.push(handle);
}
