extern crate core;

use std::sync::{Arc, Mutex};
use axum::{Router, routing::get};
use axum::routing::post;
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use crate::door_handler::ParallelExecutor;

mod controllers;
mod map;
mod door_handler;
mod door;
mod weather;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let executor = ParallelExecutor::new();
    if let None = executor {
        panic!("Executor failed to initialize")
    }

    let executor = Arc::new(Mutex::new(executor.unwrap()));

    let app = Router::new()
        .route("/", get(controllers::root))
        .route("/healthz", get(controllers::get_health_status))
        .route("/jungle", get(controllers::jungle::get_status))
        .route("/router", post(controllers::router::post))
        .with_state(executor)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
