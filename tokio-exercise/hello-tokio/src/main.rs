use mini_redis::client;
use bytes;

#[tokio::main]
async fn main() -> mini_redis::Result<()>{
    let mut client = client::connect("127.0.0.1::6379").await?;
    
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?.unwrap();

    assert_eq!(result, bytes::Bytes::from("world".to_string()));

    println!("{:?}", result);

    Ok(())
}
