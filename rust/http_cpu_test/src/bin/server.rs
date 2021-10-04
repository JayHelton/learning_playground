use axum::{AddExtensionLayer, Router, body::Body, extract::Extension, handler::get, http::Response, response::{IntoResponse, Json}};
use std::{future::Future, pin::Pin, task::Poll, time::Instant};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::{Layer, Service};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct State {
    counter: usize,
}

type AppState = Arc<Mutex<State>>;

struct ResponseFuture<F> {
    future: F,
}

impl <F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
{
    type Output = Result<Response, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

#[derive(Debug, Clone)]
struct CpuAvailability<T> {
    state: AppState,
    inner: T,
}

impl<S> CpuAvailability<S> {
    fn new(state: AppState, inner: S) -> Self {
        CpuAvailability { state, inner }
    }
}

impl<S, Request> Service<Request> for CpuAvailability<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        ResponseFuture {
            future: self.inner.call(req)
        }
    }
}

#[derive(Debug, Clone)]
struct CpuAvailabilityLayer {
    state: AppState,
}

impl CpuAvailabilityLayer {
    fn new(state: AppState) -> Self {
        CpuAvailabilityLayer { state }
    }
}

impl<S> Layer<S> for CpuAvailabilityLayer {
    type Service = CpuAvailability<S>;

    fn layer(&self, service: S) -> Self::Service {
        CpuAvailability::new(self.state.clone(), service)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(State {
        counter: num_cpus::get() - 1,
    }));

    let app = Router::new()
        .route("/blocking", get(blocking_handler))
        .route("/nonblocking", get(nonblocking_handler))
        .layer(CpuAvailabilityLayer::new(state.clone()))
        .layer(AddExtensionLayer::new(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn nonblocking_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    Json(state.lock().unwrap().counter)
}

async fn blocking_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    decrement(state.clone());
    let start = Instant::now();
    loop {
        let elapsed = start.elapsed();
        if elapsed.as_secs() == 30 {
            break;
        }
    }
    increment(state);
    Json("Work Done")
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
