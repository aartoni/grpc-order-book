use std::error::Error;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub async fn connect(url: &str) -> Result<WsStream, Box<dyn Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    Ok(ws_stream)
}
