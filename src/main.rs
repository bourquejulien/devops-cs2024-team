extern crate core;

use std::sync::Arc;
use axum::{Extension, Router, routing::get};
use axum::routing::post;
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod controllers;
mod map;
mod door_handler;
mod door;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let dictionary = get_dictionary();
    if let None = dictionary {
        panic!("Cannot load dictionary");
    }

    let dictionary = Arc::new(dictionary.unwrap());

    let app = Router::new()
        .route("/", get(controllers::root))
        .route("/healthz", get(controllers::get_health_status))
        .route("/jungle", get(controllers::jungle::get_status))
        .route("/router", post(controllers::router::post))
        .layer(Extension(dictionary))
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

fn get_dictionary() -> Option<Vec<String>> {
    let response = ureq::get("https://raw.githubusercontent.com/DavidWittman/wpxmlrpcbrute/master/wordlists/1000-most-common-passwords.txt").call().ok()?;
    let words = response.into_string().ok()?.split("\n").map(|word| word.to_string()).collect::<Vec<_>>();
    return Some(words);
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
