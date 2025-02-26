use std::time::Duration;

use axum::{routing::get, Router};
use dotenv::dotenv;
use sled::Db;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = sled::open("db").expect("Failed to open database");

    let app = Router::new()
        .route("/", get(root))
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(db))
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn shutdown_signal(db: Db) {
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

    println!("🔄 Flushing database before shutdown...");
    if let Err(e) = db.flush() {
        eprintln!("⚠️ Failed to flush database: {}", e);
    }
    println!("✅ Database shutdown complete.");
}
