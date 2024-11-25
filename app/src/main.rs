use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    stream: TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, writer) = stream.into_split();
    let mut writer = FramedWrite::new(writer, LengthDelimitedCodec::new());
    let mut reader = FramedRead::new(reader, LengthDelimitedCodec::new());
    while let Some(data) = reader.next().await {
        match data {
            Ok(data) => {
                let mut msg = String::from_utf8(data.to_vec())?;
                println!("Received from {}: {}", addr, msg);
                msg.push('a');
                println!("Send data to {}: {}", addr, msg);
                writer.send("msga".into()).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    println!("Client {} disconnected", addr);
    Ok(())
}