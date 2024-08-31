use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;

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

fn main() {
    task::block_on(async {
        match read_file("example.txt").await {
            Ok(contents) => println!("File contents: {}", contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
}
