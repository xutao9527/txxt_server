use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        // 使用 tokio::spawn 为每个连接启动一个独立任务
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    mut socket: TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            println!("Connection closed by {}", addr);
            break;
        }
        buffer[n] = b'\n';
        // println!("Received from {}: {:?}", addr, &buffer[..n]);
        // println!("Send to {}: {:?}", addr, &buffer[..(n+1)]);
        socket.write_all(&buffer[..n + 1]).await?;
    }
    Ok(())
}
