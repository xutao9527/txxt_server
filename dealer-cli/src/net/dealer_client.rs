use bytes::Bytes;
use dealer::protocol::definition::packet_request::PacketRequest;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub struct DealerClient {
    server_addr: String,
    framed_writer: Option<FramedWrite<tokio::net::tcp::OwnedWriteHalf, LengthDelimitedCodec>>,
    framed_reader: Option<FramedRead<tokio::net::tcp::OwnedReadHalf, LengthDelimitedCodec>>,
}

impl DealerClient {
    pub fn new(server_addr: String) -> Self {
        Self {
            server_addr,
            framed_writer: None,
            framed_reader: None,
        }
    }

    pub async fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(&self.server_addr).await?;
        println!("Connected to server at {}", self.server_addr);
        let (reader, writer) = stream.into_split();
        self.framed_writer = Some(FramedWrite::new(writer, LengthDelimitedCodec::new()));
        self.framed_reader = Some(FramedRead::new(reader, LengthDelimitedCodec::new()));
        Ok(())
    }

    pub async fn close(&mut self) {
        self.framed_writer = None;
        self.framed_reader = None;
        println!("Connection to server closed.");
    }

    pub async fn send(&mut self, packet: PacketRequest) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(writer) = &mut self.framed_writer {
            let packet_str = serde_json::to_string(&packet)?;
            let packet_bytes = Bytes::from(packet_str);
            writer.send(packet_bytes.clone()).await?;
            println!("Sent packet: {:?}", packet_bytes);
        } else {
            return Err("Connection is not open.".into());
        }
        Ok(())
    }

    pub async fn receive(&mut self) {
        if let Some(reader) = &mut self.framed_reader {
            while let Some(response) = reader.next().await {
                match response {
                    Ok(data) => {
                        // 打印接收到的数据
                        println!("Received data: {:?}", data);
                    }
                    Err(e) => {
                        eprintln!("Failed to receive data: {}", e);
                    }
                }
            }
        }
    }
}
