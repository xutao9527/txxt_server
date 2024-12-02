use std::error::Error;

use bytes::Bytes;
use dealer::protocol::definition::packet_request::PacketRequest;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    runtime::{Builder, Runtime},
};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::log::log::SLog;

pub struct GameClient {
    rt: Runtime,
    server_addr: String,
    framed_writer: Option<FramedWrite<tokio::net::tcp::OwnedWriteHalf, LengthDelimitedCodec>>,
    framed_reader: Option<FramedRead<tokio::net::tcp::OwnedReadHalf, LengthDelimitedCodec>>,
}

impl GameClient {
    pub fn new(server_addr: String) -> Self {
        let rt = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to create runtime");
        Self {
            rt,
            server_addr,
            framed_writer: None,
            framed_reader: None,
        }
    }

    pub fn open(&mut self) -> Result<(), Box<dyn Error>> {
        let result = self.rt.block_on(async {
            let stream = TcpStream::connect(&self.server_addr).await?;
            let (reader, writer) = stream.into_split();
            self.framed_writer = Some(FramedWrite::new(writer, LengthDelimitedCodec::new()));
            self.framed_reader = Some(FramedRead::new(reader, LengthDelimitedCodec::new()));
            if let Some(framed_reader) = self.framed_reader.take() {
                tokio::spawn(async move {
                    let mut reader = framed_reader;
                    while let Some(response) = reader.next().await {
                        match response {
                            Ok(data) => {
                                SLog::info(format!("Received data: {:?}", data));
                            }
                            Err(e) => {
                                eprintln!();
                                SLog::info(format!("Failed to receive data: {}", e));
                            }
                        }
                    }
                });
            }
            Ok::<(), Box<dyn Error>>(())
        });
        if let Ok(_) = result {
            SLog::info(format!("connection successful "));
        } else {
            SLog::err(format!("connection failure :{:?}", result));
        }
        result
    }

    pub fn send(&mut self, packet: PacketRequest) -> Result<(), Box<dyn Error>> {
        let result = self.rt.block_on(async {
            if let Some(writer) = &mut self.framed_writer {
                let packet_str = serde_json::to_string(&packet)?;
                let packet_bytes = Bytes::from(packet_str);
                writer.send(packet_bytes.clone()).await?
            }
            Ok::<(), Box<dyn Error>>(())
        });
        result
    }

    pub fn close(&mut self) {
        self.framed_writer = None;
        self.framed_reader = None;
        println!("connection to server closed.");
    }
}
