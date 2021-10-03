use num_cpus;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::net::TcpListener;

struct State {
    counter: usize,
}

type AppState = Arc<Mutex<State>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .worker_threads(num_cpus::get())
        .build()?;

    runtime.block_on(async {
        let app_state = Arc::new(Mutex::new(State {
            counter: num_cpus::get() - 1,
        }));

        let addr = "127.0.0.1:8080".to_string();
        let listener = TcpListener::bind(&addr).await?;
        println!(
            "Listening on: {} with {} cores, 1 core reserved for listener",
            addr,
            num_cpus::get()
        );

        loop {
            let (_socket, _) = listener.accept().await?;
            let state = app_state.clone();
            println!(
                "New connection with {} cores left",
                available_cores(state.clone())
            );
            tokio::spawn(async move {
                handle(state);
            });
        }
    })
}

fn handle(state: AppState) {
    println!("New task started for the workers");
    if is_available(state.clone()) {
        println!("Starting Simulated Work");
        decrement(state.clone());

        let start = Instant::now();
        loop {
            let elapsed = start.elapsed();
            if elapsed.as_secs() == 15 {
                break;
            }
        }

        increment(state.clone());
        println!("Finished Simulated Work");
    } else {
        println!("Cannot process request");
    }
}

fn available_cores(state: AppState) -> usize {
    let handle = state.lock().unwrap();
    handle.counter
}

fn is_available(state: AppState) -> bool {
    let handle = state.lock().unwrap();
    handle.counter != 0
}

fn decrement(state: AppState) {
    let mut handle = state.lock().unwrap();
    handle.counter = handle.counter - 1;
}

fn increment(state: AppState) {
    let mut handle = state.lock().unwrap();
    handle.counter = handle.counter + 1;
}
