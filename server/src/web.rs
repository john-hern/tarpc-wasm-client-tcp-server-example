use async_stream::stream;
use futures::Stream;
use log::info;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tarpc::serde_transport::Transport;
use tokio::stream::StreamExt;
use tokio_serde::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
//use tarpc::Transport;
use async_tungstenite::tokio::accept_async;
use std::marker::Unpin;
use ws_stream_tungstenite::*;

pub async fn bind<Item, SinkItem, Codec, CodecFn>(
    codec_fn: CodecFn,
) -> Option<
    impl Stream<
        Item = Result<
            Transport<
                ws_stream_tungstenite::WsStream<
                    async_tungstenite::tokio::TokioAdapter<tokio::net::TcpStream>,
                >,
                Item,
                SinkItem,
                Codec,
            >,
            std::io::Error,
        >,
    >,
>
where
    Item: for<'de> Deserialize<'de> + Unpin,
    SinkItem: Serialize + Unpin,
    Codec: Serializer<SinkItem> + Deserializer<Item> + Unpin,
    CodecFn: Fn() -> Codec,
{
    info!("Binding RPC TCP Session");

    //Setup the basic args for the socket.
    let ip: Ipv4Addr = "127.0.0.1".parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), 8083);

    //Create the socket
    let stream = stream! {
        let mut listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        info!("Bound, waiting on clients");
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    info!("WS Peer connected");
                    let addr = stream
                        .peer_addr()
                        .expect("connected streams should have a peer address");
                    info!("Peer address: {}", addr);
                    let ws = accept_async(stream).await.unwrap();
                    let ws_stream = WsStream::new(ws);
                    info!("New WebSocket connection: {}", addr);
                    let frame = Framed::new(ws_stream, LengthDelimitedCodec::new());
                    let tmp = tarpc::serde_transport::new(frame, codec_fn());
                    yield Ok(tmp)
                }
                Err(_e) => { /* connection failed */ }
            }
        }
    };
    //pin_mut!(stream);
    Some(stream)
}
