use futures_util::{SinkExt, StreamExt};
use snowflake::SnowflakeIdGenerator;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use dealer::protocol::processor::packet_processor::process_packet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        let mut id_generator = SnowflakeIdGenerator::new(1, 1);
        tokio::spawn(async move {
            id_generator.lazy_generate();
            if let Err(e) = handle_client(stream, addr,id_generator.lazy_generate()).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}


async fn handle_client(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    clientId: i64
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, writer) = stream.into_split();
    let mut writer = FramedWrite::new(writer, LengthDelimitedCodec::new());
    let mut reader = FramedRead::new(reader, LengthDelimitedCodec::new());

    while let Some(data) = reader.next().await {
        match data {
            Ok(data) => {
                process_packet(&data).await;
                let mut msg = String::from_utf8(data.to_vec())?;
                println!("Received from {}: {}", addr, msg);
                // msg.push('a');
                // println!("Send data to {}: {}", addr, msg);
                // writer.send("msga".into()).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    println!("Client {} disconnected", addr);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;
    use snowflake::SnowflakeIdGenerator;

    #[test]
    fn test_snowflake() {
        let mut id_generator = SnowflakeIdGenerator::new(1, 1); // 初始化 Snowflake ID 生成器
        let start_time = Instant::now();
        let mut count = 0;

        // 用于存储生成的 ID
        let mut generated_ids = Vec::with_capacity(100000000);

        // 生成 ID，不进行实时去重检查
        while start_time.elapsed().as_secs() < 2 {
            let id = id_generator.lazy_generate(); // 生成 ID
            generated_ids.push(id); // 将 ID 添加到数组中
            count += 1;
        }

        // 输出生成的 ID 数量
        println!("Generated {} IDs in 2 seconds", count);

        // 后期去重检查：将所有生成的 ID 插入到 HashSet 中
        let unique_ids: HashSet<_> = generated_ids.into_iter().collect();
        let total_unique_ids = unique_ids.len();

        if total_unique_ids < count {
            println!("Duplicate IDs found. {} duplicates detected.", count - total_unique_ids);
        } else {
            println!("No duplicates found.");
        }
    }
}

