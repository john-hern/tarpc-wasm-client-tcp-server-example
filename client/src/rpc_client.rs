
use async_io_stream::IoStream;



use log::{error, info, warn};

use serde::{Deserialize, Serialize};

use std::marker::Unpin;




use tarpc::serde_transport::Transport;

use tokio_serde::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use ws_stream_wasm::*;



pub async fn connect<Item, SinkItem, Codec, CodecFn>(
    codecFn: CodecFn,
) -> Result<Transport<IoStream<WsStreamIo, Vec<u8>>, Item, SinkItem, Codec>, std::io::Error>
where
    Item: for<'de> Deserialize<'de>,
    SinkItem: Serialize,
    Codec: Serializer<SinkItem> + Deserializer<Item>,
    CodecFn: Fn() -> Codec,
{
    info!("Starting connect2");
    let _url = "127.0.0.1:8083";
    info!("Connecting to server: {}", "127.0.0.1:8083");
    match WsMeta::connect("ws://127.0.0.1:8083", None).await {
        Ok((_ws, _wsio)) => {
            //let session = WebSocketSession::connect(url);
            info!("Creating the frame");
            let frame = Framed::new(_wsio.into_io(), LengthDelimitedCodec::new());
            info!("Creating the Transport");
            let tmp = tarpc::serde_transport::new(frame, codecFn());
            info!("Returning Transport");
            Ok(tmp)
        }
        Err(e) => {
            info!("Errored on WsMeta connect\n{:?}", e);
            Err(std::io::Error::from(std::io::ErrorKind::ConnectionRefused))
        }
    }
}

pub async fn build_client<Item, SinkItem>(
) -> Result<impl tarpc::Transport<SinkItem, Item>, std::io::Error>
where
    Item: for<'de> Deserialize<'de> + Unpin,
    SinkItem: Serialize + Unpin,
{
    info!("In build client");
    Ok(
        connect(tokio_serde::formats::Json::<Item, SinkItem>::default)
            .await
            .unwrap(),
    )
    //self.server_handles.push(handle);
}
