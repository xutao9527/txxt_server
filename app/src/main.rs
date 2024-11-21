use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        // 使用 tokio::spawn 为每个连接启动一个独立任务
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
    let  _ = tokio::io::BufWriter::new(writer);
    let mut reader = FramedRead::new(reader, LengthDelimitedCodec::new());

    while let Some(data) = reader.next().await {
        match data {
            Ok(data) => {
                let msg = String::from_utf8(data.to_vec())?;
                println!("Received from {}: {}", addr, msg);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
