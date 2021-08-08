use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;
    // set a key
    client.set("hello", "world".into()).await?;
    // get the key
    let result = client.get("hello").await?;
    println!("got value from server; result={:?}", result);
    Ok(())
}
