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

#[cfg(test)]
mod tests {
    use tokio_util::codec::Encoder;
    #[test]
    fn encode() {
        let mut codec = tokio_util::codec::LengthDelimitedCodec::new();
        let data = bytes::Bytes::from("123456789123123456789123123456789123");
        let mut dst = bytes::BytesMut::new();
        let _ = codec.encode(data, &mut dst);
        let result: Vec<u8> = dst.to_vec();
        let binary_output: String = result
            .iter()
            .map(|byte| format!("{:08b}", byte)) // 每个字节格式化为8位二进制
            .collect::<Vec<String>>()
            .join(" ");
        println!("Binary output: {}", binary_output);
        let hex_output: String = result
            .iter()
            .map(|byte| format!("{:02x}", byte)) // 每个字节格式化为两位十六进制
            .collect::<Vec<String>>()
            .join("");
        println!("Hexadecimal output: {}", hex_output);
    }
}
