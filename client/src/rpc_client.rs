use ::rpc::services::PingServiceClient; 
use async_stream::stream;
use futures::Stream;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio_serde::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio::io::{AsyncRead, AsyncWrite};
use std::pin::*;
use std::task::{Context, Poll};
use futures::{ready};
use pin_project::*;
use tarpc::serde_transport::Transport;
//use tarpc::Transport;
use std::marker::Unpin;
use yew::services::{WebSocketService, websocket::{WebSocketTask, WebSocketStatus}};
use yew::callback::Callback;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::iter::Iterator;
use ws_stream_wasm::*;
use async_io_stream::IoStream;

pub enum WebSocketSessionState { 
    Initialized, 
    Connected,
    Closed, 
    Errored(anyhow::Error)
}
#[pin_project]
pub struct WebSocketSession{ 
    #[pin]
    inner: Option<WebSocketTask>,
    buf: Arc<RwLock<Vec<u8>>>,
    closed: bool,
    state: Arc<RwLock<WebSocketSessionState>>
}

impl WebSocketSession { 
    fn new() -> Self { 
        Self { 
            inner: None,
            buf: Arc::new(RwLock::new(Vec::new())),
            closed: true,
            state: Arc::new(RwLock::new(WebSocketSessionState::Initialized))
        }
    }


    pub fn connect(url: &'static str) -> Self
    { 
        let mut session = WebSocketSession::new();
        session._connect(url);
        session
    }
    pub fn closed(&self) -> bool { 
        return self.closed();
    }
    fn _connect(&mut self, url: &'static str) { 
        
        let mut _buf = self.buf.clone();
        let callback = Callback::<Result<Vec<u8>, anyhow::Error>>::Callback(Rc::new(move |x|{ 
            let mut buf = _buf.clone();
            match x { 
                Ok(msg) => {
                    buf.write().unwrap().extend(&msg);
                },
                Err(_e) => {}
            }
        }));
        let _state = self.state.clone();

        let notification  = Callback::<WebSocketStatus>::Callback(Rc::new(|x: WebSocketStatus|{
            
        }));
        let connection = WebSocketService::connect_binary(url, 
                                                          callback, 
                                                          notification).unwrap(); 
        self.inner = Some(connection);
        self.closed = false;
    }
}

impl AsyncRead for WebSocketSession
{ 
     /// Attempts to read from the `AsyncRead` into `buf`.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_read))`.
    ///
    /// If no data is available for reading, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object becomes
    /// readable or is closed.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> { 
        let mut bytes = self.buf.write().unwrap();
        let len = bytes.len();
        if len == 0 { 
            Poll::Pending
        }else if self.closed(){ 
            Poll::Ready(Ok(0))
        }else { 
            let i: Vec<u8> = bytes.drain(..).collect();
            buf.copy_from_slice(&i);
            Poll::Ready(Ok(len))
        }
    }
}

impl AsyncWrite for WebSocketSession { 
     /// Attempt to write bytes from `buf` into the object.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_written))`.
    ///
    /// If the object is not ready for writing, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object becomes
    /// writable or is closed.
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> { 
        if let Some(ref mut connection) = self.inner { 
            let v = buf.to_vec(); 
            let len = v.len();
            connection.send_binary(Ok(v));
            Poll::Ready(Ok(len))
        }else if self.closed() { 
            Poll::Ready(Ok(0))
        }else{ 
            Poll::Pending
        }
    }
    /// Attempts to flush the object, ensuring that any buffered data reach
    /// their destination.
    ///
    /// On success, returns `Poll::Ready(Ok(()))`.
    ///
    /// If flushing cannot immediately complete, this method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object can make
    /// progress towards flushing.
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> { 
        Poll::Ready(Ok(()))
    }

    /// Initiates or attempts to shut down this writer, returning success when
    /// the I/O connection has completely shut down.
    ///
    /// This method is intended to be used for asynchronous shutdown of I/O
    /// connections. For example this is suitable for implementing shutdown of a
    /// TLS connection or calling `TcpStream::shutdown` on a proxied connection.
    /// Protocols sometimes need to flush out final pieces of data or otherwise
    /// perform a graceful shutdown handshake, reading/writing more data as
    /// appropriate. This method is the hook for such protocols to implement the
    /// graceful shutdown logic.
    ///
    /// This `shutdown` method is required by implementers of the
    /// `AsyncWrite` trait. Wrappers typically just want to proxy this call
    /// through to the wrapped type, and base types will typically implement
    /// shutdown logic here or just return `Ok(().into())`. Note that if you're
    /// wrapping an underlying `AsyncWrite` a call to `shutdown` implies that
    /// transitively the entire stream has been shut down. After your wrapper's
    /// shutdown logic has been executed you should shut down the underlying
    /// stream.
    ///
    /// Invocation of a `shutdown` implies an invocation of `flush`. Once this
    /// method returns `Ready` it implies that a flush successfully happened
    /// before the shutdown happened. That is, callers don't need to call
    /// `flush` before calling `shutdown`. They can rely that by calling
    /// `shutdown` any pending buffered data will be written out.
    ///
    /// # Return value
    ///
    /// This function returns a `Poll<io::Result<()>>` classified as such:
    ///
    /// * `Poll::Ready(Ok(()))` - indicates that the connection was
    ///   successfully shut down and is now safe to deallocate/drop/close
    ///   resources associated with it. This method means that the current task
    ///   will no longer receive any notifications due to this method and the
    ///   I/O object itself is likely no longer usable.
    ///
    /// * `Poll::Pending` - indicates that shutdown is initiated but could
    ///   not complete just yet. This may mean that more I/O needs to happen to
    ///   continue this shutdown operation. The current task is scheduled to
    ///   receive a notification when it's otherwise ready to continue the
    ///   shutdown operation. When woken up this method should be called again.
    ///
    /// * `Poll::Ready(Err(e))` - indicates a fatal error has happened with shutdown,
    ///   indicating that the shutdown operation did not complete successfully.
    ///   This typically means that the I/O object is no longer usable.
    ///
    /// # Errors
    ///
    /// This function can return normal I/O errors through `Err`, described
    /// above. Additionally this method may also render the underlying
    /// `Write::write` method no longer usable (e.g. will return errors in the
    /// future). It's recommended that once `shutdown` is called the
    /// `write` method is no longer called.
    ///
    /// # Panics
    ///
    /// This function will panic if not called within the context of a future's
    /// task.
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> { 
        self.closed = true;
        Poll::Ready(Ok(()))
    }
}

//#[cfg(not(feature = "web"))]
pub async fn connect<Item, SinkItem, Codec, CodecFn>(
    codecFn: CodecFn,
) -> Result<Transport<WebSocketSession, Item, SinkItem, Codec>, std::io::Error>
where
    Item: for<'de> Deserialize<'de>,
    SinkItem: Serialize,
    Codec: Serializer<SinkItem> + Deserializer<Item>,
    CodecFn: Fn() -> Codec,
{
    let url = "ws://127.0.0.1:8083";
    info!("Connecting to server: {}", "127.0.0.1:8083");
    
    let session = WebSocketSession::connect(url);
                
    let frame = Framed::new(session, LengthDelimitedCodec::new());
    let tmp = tarpc::serde_transport::new(frame, codecFn());
    Ok(tmp)
}


pub async fn connect2<Item, SinkItem, Codec, CodecFn>(
    codecFn: CodecFn,
) -> Result<Transport<IoStream<WsStreamIo, Vec<u8>>, Item, SinkItem, Codec>, std::io::Error>
where
    Item: for<'de> Deserialize<'de>,
    SinkItem: Serialize,
    Codec: Serializer<SinkItem> + Deserializer<Item>,
    CodecFn: Fn() -> Codec,
{
    info!("Starting connect2");
    let url = "127.0.0.1:8083";
    info!("Connecting to server: {}", "127.0.0.1:8083");
    match WsMeta::connect( "ws://127.0.0.1:8083", None ).await { 
        Ok((ws, _wsio)) => { 
            //let session = WebSocketSession::connect(url);
            info!("Creating the frame");
            let frame = Framed::new(_wsio.into_io(), LengthDelimitedCodec::new());
            info!("Creating the Transport");
            let tmp = tarpc::serde_transport::new(frame, codecFn());
            info!("Returning Transport");
            Ok(tmp)
        }, 
        Err(e) => { 
            info!("Errored on WsMeta connect\n{:?}", e);
            Err(std::io::Error::from(std::io::ErrorKind::ConnectionRefused))
        }

    }
    
}



pub async fn build_client<Item, SinkItem>() -> Result<impl tarpc::Transport<SinkItem, Item>, std::io::Error>
    where
        Item: for<'de> Deserialize<'de> + Unpin,
        SinkItem: Serialize + Unpin,
{ 
    info!("In build client");
    Ok(connect2(tokio_serde::formats::Json::<Item, SinkItem>::default).await.unwrap())
    //self.server_handles.push(handle);
}
