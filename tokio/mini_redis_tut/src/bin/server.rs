#![allow(unused_doc_comments)]

use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::io;
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command::{self, Set, Get}, Connection, Frame};

/**
 * Create a shared type.
 *
 * Arc (atomically referenced counter) is a thread safe pointer.
 * It returns a handler for a shared value on the heap and handlers
 * references internally through a counter.
 *
 * More or less like a smart pointer in c++ that tracks across threads.
 *
 * The mutex is of course used to ensure integrity when multiple threads
 * are accessing data. It can cause slowness if many tasks are waiting for the 
 * lock to opening up on the state.
 */
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?; 
    let db = Arc::new(Mutex::new(HashMap::new()));

    /**
     * Loop and accept connections, spawn a new task to keep things concurrent
     *
     * Tokio tasks are async green threads. Spawn does return a handler if we wanted to await it
     * for some reason.
     *
     * Tasks are managed by the tokio scheduler. Each spawn submits a new task. 
     * It could be run on the same thread as spawned, or in a different one. 
     * They can also be moved between threads
     */
    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) { 
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }           
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
