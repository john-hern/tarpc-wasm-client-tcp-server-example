use async_stream::stream;
use futures::Stream;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;
use tokio_serde::*;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio::net::{TcpListener, TcpStream};
use tarpc::serde_transport::Transport;
//use tarpc::Transport;
use std::marker::Unpin;
use tokio_tungstenite::*;
use tokio::io::{AsyncRead, AsyncWrite };
use bytes::{Bytes, BytesMut};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_sink::Sink;
use futures::{ready};
use pin_project::*;
use tungstenite::{Message};
use tungstenite::error::Error as WsError;
use ws_stream_tungstenite::*;
use async_tungstenite::tokio::accept_async;


#[pin_project]
pub struct WebSocketSession { 
    #[pin]
    inner: WebSocketStream<TcpStream>
}

impl WebSocketSession 
{ 
    pub fn new(inner: WebSocketStream<TcpStream>) -> Self { 
        Self { 
            inner
        }
    }
    pub async fn test(&mut self) { 
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
        match self.project().inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(Ok(0)),
            Poll::Ready(Some(Ok::<_, _>(next))) => {
               
                match next { 
                    Message::Binary(bytes) => { 
                        buf.clone_from_slice(&bytes);
                        return Poll::Ready(Ok(bytes.len()));
                    },
                    Message::Ping(bytes) => { 
                        Poll::Pending
                    },
                    _=> { 
                        Poll::Pending
                    }
                }
            },
            Poll::Ready(Some(Err::<_, _>(e))) => {
                Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, e)))
            }
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
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> { 
        let v = buf.to_vec(); 
        let len = v.len();
        match self.project().inner.start_send(Message::Binary(v)) { 
            Ok(_) => Poll::Ready(Ok(len)),
            Err(e) => Poll::Ready(Err(std::io::Error::from(std::io::ErrorKind::Other)))
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
        match self.project().inner.poll_flush(cx){ 
            Poll::Pending => Poll::Pending, 
            Poll::Ready(Ok(_value)) => Poll::Ready(Ok(())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(std::io::Error::from(std::io::ErrorKind::Other)))
        }
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
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> { 
        match self.project().inner.poll_close(cx){ 
            Poll::Pending => Poll::Pending, 
            Poll::Ready(Ok(_value)) => Poll::Ready(Ok(())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(std::io::Error::from(std::io::ErrorKind::Other)))
        }
    }
}


/*

#[pin_project]
pub struct WebSocketSession<T> { 
    #[pin]
    inner: WebSocketStream<T>
}
impl<T> WebSocketSession<T> 
    where 
        T: AsyncRead + AsyncWrite + Unpin
{ 
    pub fn new(inner: WebSocketStream<T>) -> Self { 
        Self { 
            inner
        }
    }
    pub async fn test(&mut self) { 
    }
}
impl<T> Stream for WebSocketSession<T> 
    where 
        T: AsyncRead + AsyncWrite + Unpin
{ 
    type Item = Result<BytesMut, std::io::Error>;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project().inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok::<_, _>(next))) => {
                if next.is_binary() 
                { 
                    if let Message::Binary(bytes) = next { 
                        return Poll::Ready(Some(Ok(BytesMut::from(bytes.as_slice()))));
                    } 
                }
                Poll::Pending
            },
            Poll::Ready(Some(Err::<_, _>(e))) => {
                Poll::Ready(Some(Err(std::io::Error::new(std::io::ErrorKind::Other, e))))
            }
        }
    }
}

impl<T> Sink<Bytes> for WebSocketSession<T>
    where 
        T: AsyncRead + AsyncWrite + Unpin
{
    type Error = tungstenite::error::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        let v = item.to_vec(); 
        
        self.project().inner.start_send(Message::Binary(v))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}
*/
//#[cfg(not(feature = "web"))]
pub async fn bind<Item, SinkItem, Codec, CodecFn>(
    codecFn: CodecFn,
) -> Option<impl Stream<Item = Result<Transport<WebSocketSession, Item, SinkItem, Codec>, std::io::Error>>>
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
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    info!("WS Peer connected");
                    let addr = stream
                        .peer_addr()
                        .expect("connected streams should have a peer address");
                    info!("Peer address: {}", addr);
                    let mut ws_stream = tokio_tungstenite::accept_async(stream)
                        .await
                        .expect("Error during the websocket handshake occurred");
                    info!("New WebSocket connection: {}", addr);   

                    let session = WebSocketSession::new(ws_stream);    
                    let frame = Framed::new(session, LengthDelimitedCodec::new());
                    let tmp = tarpc::serde_transport::new(frame, codecFn());
                    info!("Yeilding the session");
                    yield Ok(tmp)
                }
                Err(e) => { /* connection failed */ }
            }
        }
    };
    //pin_mut!(stream);
    Some(stream)
}

pub async fn bind2<Item, SinkItem, Codec, CodecFn>(
    codecFn: CodecFn,
) -> Option<impl Stream<Item = Result<Transport<ws_stream_tungstenite::WsStream<async_tungstenite::tokio::TokioAdapter<tokio::net::TcpStream>>, Item, SinkItem, Codec>, std::io::Error>>>
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
                    let mut ws = accept_async(stream).await.unwrap();
                    let mut ws_stream = WsStream::new(ws);
                    info!("New WebSocket connection: {}", addr);     
                    let frame = Framed::new(ws_stream, LengthDelimitedCodec::new());
                    let tmp = tarpc::serde_transport::new(frame, codecFn());
                    yield Ok(tmp)
                }
                Err(e) => { /* connection failed */ }
            }
        }
    };
    //pin_mut!(stream);
    Some(stream)
}