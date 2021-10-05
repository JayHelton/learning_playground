use axum::{
    extract::Extension,
    handler::get,
    http::StatusCode,
    response::{IntoResponse, Json},
    AddExtensionLayer, Router,
};
use pin_project::pin_project;
use std::{borrow::Cow, convert::Infallible, future::Future, pin::Pin, task::Poll, time::Instant};
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
type BoxError = Box<dyn std::error::Error + Send + Sync>;

/**
 * We define a new error for rejections.
 * We implement std::error::Error so that we can convert
 * the error back down to a general error.
 */
#[derive(Debug, Default)]
struct CoresUnavailable(());

impl std::fmt::Display for CoresUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("No cores available")
    }
}

impl std::error::Error for CoresUnavailable {}

/**
 * For complex middleware, we can define our own futures.
 * We probably could also just return a Box dyn in the Service impl
 */
#[pin_project]
struct ResponseFuture<F> {
    #[pin]
    future: F,
    cores_available: bool,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if !self.cores_available {
            let err = Box::new(CoresUnavailable(()));
            return Poll::Ready(Err(err));
        }
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }
        Poll::Pending
    }
}

/**
 * CPU availability middleware. It will call the inner service if there are workers available,
 * else it will throw an error
 */
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
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!(
            "Checking CPUs: {} available",
            available_cores(self.state.clone())
        );
        ResponseFuture {
            cores_available: is_available(self.state.clone()),
            future: self.inner.call(req),
        }
    }
}

/**
 * Middleware Layer for the service decoration to make it a reusable component
 */
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
        .layer(CpuAvailabilityLayer::new(state.clone()))
        .layer(AddExtensionLayer::new(state))
        .handle_error(|error: BoxError| {
            if error.is::<CoresUnavailable>() {
                return Ok::<_, Infallible>((
                    StatusCode::TOO_MANY_REQUESTS,
                    Cow::from(format!("All cores are busy. Try again later.")),
                ));
            }

            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::from(format!("Unhandled internal error: {}", error)),
            ))
        });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn blocking_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    println!("Starting Work");
    decrement(state.clone());
    let start = Instant::now();
    loop {
        let elapsed = start.elapsed();
        if elapsed.as_secs() == 30 {
            break;
        }
    }
    increment(state);
    println!("Work Done");
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
