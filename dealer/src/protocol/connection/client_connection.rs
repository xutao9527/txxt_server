use tokio::net::tcp::OwnedWriteHalf;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

#[derive(Debug)]
pub struct ClientConnection{
    pub client_id:i64,
    pub addr: std::net::SocketAddr,
    pub authentication:bool,
    pub writer: FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>,
}