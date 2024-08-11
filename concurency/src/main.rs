use std::fs::File;
use std::io::{self, Read};

fn read_file_sync(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    println!("Task 1 started");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Task 1 completed");

    println!("Task 2 started");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Task 2 completed");

    let content = read_file_sync("example.txt")?;
    println!("File content: {}", content);
    Ok(())
}

//In this program, 
// Task 2 will only start after Task 1 is completed, 
// resulting in a total wait time of 4 seconds.
// the program will wait for the file to be fully read before printing its content.
//  If the file reading takes a long time, the entire program will be blocked.