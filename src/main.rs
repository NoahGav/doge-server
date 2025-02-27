use std::{env, time::Duration};

use anyhow::Result;
use axum::{extract::State, routing::get, Router};
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
    // Get the saved server_knowledge.
    let server_knowledge: Option<i64> = db
        .get("txn-server-knowledge")
        .ok()
        .flatten()
        .map(|x| bincode::deserialize(&x).unwrap());

    let response =
        apis::transactions_api::get_transactions(config, budget_id, None, None, server_knowledge)
            .await?;

    // Update the server knowledge.
    db.insert(
        "txn-server-knowledge",
        bincode::serialize(&response.data.server_knowledge)?,
    )?;

    log::info!("🔄 Syncing transactions...");

    for txn in response.data.transactions {
        log::info!("🔄 txn:{}", txn.id);
        let key = format!("txn:{}", txn.id);
        db.insert(key, bincode::serialize(&txn)?)?;
    }

    log::info!("✅ Syncing transactions complete.");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    dotenv()?;

    let db = sled::open("db").expect("⚠️ Failed to open database");
    let ynab_access_token = env::var("YNAB_ACCESS_TOKEN")?;
    let ynab_budget_id = env::var("YNAB_BUDGET_ID")?;

    tokio::spawn({
        let db = db.clone();

        async move {
            let mut config = Configuration::new();
            config.bearer_access_token = Some(ynab_access_token);

            loop {
                sync_transactions(&config, &ynab_budget_id, &db)
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10 * 60)).await;
            }
        }
    });

    let api_router = Router::new().route("/transactions", get(transactions));

    let static_router = Router::new().route("/", get(root));

    let app = Router::new()
        .nest("/api", api_router)
        .merge(static_router)
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(db))
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn transactions(State(db): State<Db>) -> String {
    let txns = list_transactions(&db).collect::<Vec<_>>();
    serde_json::to_string(&txns).unwrap()
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

    log::info!("🔄 Flushing database before shutdown...");
    if let Err(e) = db.flush() {
        log::error!("⚠️ Failed to flush database: {}", e);
    }
    log::info!("✅ Database shutdown complete.");
}

// TODO: Clean up and put in different modules and stuff.
// TODO: Start the sveltekit site.
