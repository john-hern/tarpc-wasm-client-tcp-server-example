use ::server::web::bind2;
use ::rpc::services as services;
use tokio::prelude::*;
use tarpc;
use serde::{Deserialize, Serialize};
use std::pin::*;
use services::PingService;
use ::server::service_impl::PingServiceImpl;
use tarpc::service;
use tarpc::server::*;
use tokio::stream::*;
use futures_util::*;
use futures_util::StreamExt;
use tarpc::context::*;
use tarpc::rpc::*;
use tarpc::*;
use log::{info};
use ::server::loggin::LogWriter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let log_writer = LogWriter::new(log::Level::Info);
    if let Ok(()) = log::set_boxed_logger(Box::new(log_writer)) {
        log::set_max_level(log::LevelFilter::Info)
    }
    info!("First Message");

    let tmp = build_server()
    .await
    .expect("Failed to get server channel");
    let tmp = tmp.map_ok(move |x| {
        info!("Mapping the client session"); 
        let channel = tarpc::server::BaseChannel::with_defaults(x);
        let server = PingServiceImpl{}; 
        info!("Spawning client channel");
        tokio::spawn(channel.respond_with(server.serve()).execute())
    });

    //TODO: Will likely need a way to kill the connection. Need to figure that out. 
    let handle = tokio::spawn(tmp.for_each(|_| async {}));
    handle.await;
    Ok(())
}

async fn build_server<Item, SinkItem>() -> Option<impl Stream<Item = Result<impl tarpc::Transport<SinkItem, Item>, std::io::Error>>>
    where
        Item: for<'de> Deserialize<'de> + Unpin,
        SinkItem: Serialize + Unpin,
{ 
    Some(bind2(tokio_serde::formats::Json::<Item, SinkItem>::default).await.unwrap())

    //self.server_handles.push(handle);
}

