use std::{env, time::Duration};

use anyhow::Result;
use axum::{routing::get, Router};
use dotenv::dotenv;
use sled::Db;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use ynab_rs::{
    apis::{self, configuration::Configuration},
    models::TransactionDetail,
};

fn list_transactions(db: &Db) -> impl Iterator<Item = TransactionDetail> {
    db.scan_prefix("txn:")
        .filter_map(|x| x.ok())
        .map(|(_key, value)| bincode::deserialize(&value).unwrap())
}

async fn sync_transactions(config: &Configuration, budget_id: &str, db: &Db) -> Result<()> {
    // TODO: Check the db for the txn server knowledge number.
    // TODO: If it is present then include it in the request.
    // TODO: Make sure to update the server knowledge number.

    let response =
        apis::transactions_api::get_transactions(config, budget_id, None, None, None).await?;

    for txn in response.data.transactions {
        let key = format!("txn:{}", txn.id);
        db.insert(key, bincode::serialize(&txn)?)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let db = sled::open("db").expect("Failed to open database");
    let ynab_access_token = env::var("YNAB_ACCESS_TOKEN")?;
    let ynab_budget_id = env::var("YNAB_BUDGET_ID")?;

    let mut config = Configuration::new();
    config.bearer_access_token = Some(ynab_access_token);

    // TODO: Create a background thread for syncing data.
    // sync_transactions(&config, &ynab_budget_id, &db).await?;

    for txn in list_transactions(&db) {
        println!("{:?}", txn);
    }

    let app = Router::new()
        .route("/", get(root))
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(db))
        .await?;

    Ok(())
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
