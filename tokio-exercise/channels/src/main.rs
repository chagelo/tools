use mini_redis::client;
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};


#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // Clone a `tx` handle for the second f
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // Open a connectiont to mini-redis address.
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                },
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore erros
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { key: "rust".to_string(), resp: resp_tx };


        // Send the Get request
        if tx.send(cmd).await.is_err() {
            eprint!("connection task shutdown!");
            return;
        }

        // Await the response
        let res = resp_rx.await.unwrap();
        println!("Got (Get)={:?}, {:?}", res, res.as_ref().unwrap().is_none());
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set { key: "hello".to_string(), val: "world".into(), resp: resp_tx };


        // Send the Get request
        if tx2.send(cmd).await.is_err() {
            eprint!("connection task shutdown!");
            return;
        }

        // Await the response
        let res = resp_rx.await.unwrap();
        println!("Got (Set)={:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
