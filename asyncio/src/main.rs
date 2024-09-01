use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;
use async_std::net::TcpStream;

async fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    task::block_on(async {
        match read_file("example.txt").await {
            Ok(contents) => println!("File contents: {}", contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
        
    });

    task::block_on(async {
        if let Err(e) = write_file("output.txt", "Hello, async world!").await {
            eprintln!("Error writing to file: {}", e);
        }
    });

    task::block_on(connect_to_server())
}

async fn connect_to_server() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"Hello, server!").await?;
    
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
