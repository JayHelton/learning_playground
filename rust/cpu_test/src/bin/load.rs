use std::thread;
use std::time;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    loop {
        println!("Spawning new client");
        tokio::spawn(async move {
            let addr = "127.0.0.1:8080".to_string();
            TcpStream::connect(addr).await
        });
        thread::sleep(time::Duration::from_secs(1));
    }
}
