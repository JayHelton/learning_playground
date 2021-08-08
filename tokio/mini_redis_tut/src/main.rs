use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // bind a listener to a port
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        // a new task is spawned for each inbound socket, which is moved to a new task
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    // this is provided by mini redis for making connections
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        // more mini redis lib stuff
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
