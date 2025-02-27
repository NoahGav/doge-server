use chrono::{Duration, Months, NaiveDate, Weekday};
use server::budget::{Money, MoneyFlow, When};

fn main() {
    let flows = [
        // VA
        MoneyFlow {
            amount: Money::dollars(4044.00),
            when: When::FederalPaycheck(1),
        },
        // SSA Dad
        MoneyFlow {
            amount: Money::dollars(1365.00),
            when: When::FederalPaycheck(3),
        },
        // SSA Mom
        MoneyFlow {
            amount: Money::dollars(756.00),
            when: When::FederalPaycheck(3),
        },
        // USPS
        MoneyFlow {
            amount: Money::dollars(2450.0),
            when: When::EveryTwoWeeks(NaiveDate::from_ymd_opt(2025, 2, 26).unwrap()),
        },
        // Mortgage
        MoneyFlow {
            amount: Money::dollars(-1660.70),
            when: When::SpecificDay(1),
        },
        // Ferrel Gas
        MoneyFlow {
            amount: Money::dollars(-330.00),
            when: When::SpecificDay(1),
        },
        // Student Loan
        MoneyFlow {
            amount: Money::dollars(-157.00),
            when: When::SpecificDay(1),
        },
        // Storage
        MoneyFlow {
            amount: Money::dollars(-90.00),
            when: When::SpecificDay(1),
        },
        // Internet
        MoneyFlow {
            amount: Money::dollars(-170.0),
            when: When::SpecificDay(5),
        },
        // BENEFEDS
        MoneyFlow {
            amount: Money::dollars(-20.00),
            when: When::SpecificDay(7),
        },
        // Trash
        MoneyFlow {
            amount: Money::dollars(-63.00),
            when: When::SpecificDay(10),
        },
        // AT&T
        MoneyFlow {
            amount: Money::dollars(-610.00),
            when: When::SpecificDay(22),
        },
        // Electricity
        MoneyFlow {
            amount: Money::dollars(-500.00),
            when: When::SpecificDay(22),
        },
        // OneMain Loan
        MoneyFlow {
            amount: Money::dollars(-340.00),
            when: When::SpecificDay(10),
        },
        // Hilux Payment
        MoneyFlow {
            amount: Money::dollars(-740.00),
            when: When::LastDayOfMonth,
        },
        // Progressive
        MoneyFlow {
            amount: Money::dollars(-592.00),
            when: When::SpecificDay(4),
        },
        // USAA Insurance
        MoneyFlow {
            amount: Money::dollars(-128.21),
            when: When::SpecificDay(3),
        },
        // Groceries
        MoneyFlow {
            amount: Money::dollars(-300.00),
            when: When::EveryWeekday(Weekday::Wed),
        },
        // Commodities
        MoneyFlow {
            amount: Money::dollars(-100.00),
            when: When::EveryWeekday(Weekday::Wed),
        },
        // Gas
        MoneyFlow {
            amount: Money::dollars(-31.15),
            when: When::EveryDay,
        },
    ];

    let mut date = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();
    let end_date = date + Months::new(6);
    let mut money = Money::dollars(5489.51);

    while date < end_date {
        for flow in flows {
            if flow.when.matches(date) {
                money.add(flow.amount);
            }
        }

        println!("{}: {}", date, money);
        date += Duration::days(1);
    }
}

// use chrono::{Duration, NaiveDate};
// use server::budget::When;

// fn main() {
//     let rule1 = When::SpecificDay(1);
//     let rule2 = When::LastDayOfMonth;
//     let rule3 = When::EveryTwoWeeks(NaiveDate::from_ymd_opt(2025, 2, 26).unwrap());
//     let rule4 = When::FederalPaycheck(1);

//     let mut date = NaiveDate::from_ymd_opt(2025, 2, 1).unwrap();
//     let end_date = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();

//     while date < end_date {
//         println!("{}", date);
//         println!("rule1: {}", rule1.matches(date));
//         println!("rule2: {}", rule2.matches(date));
//         println!("rule3: {}", rule3.matches(date));
//         println!("rule4: {}", rule4.matches(date));
//         date += Duration::days(1);
//     }
// }

// use std::{env, time::Duration};

// use anyhow::Result;
// use axum::{extract::State, routing::get, Router};
// use dotenv::dotenv;
// use sled::Db;
// use tokio::signal;
// use tower_http::{services::ServeDir, timeout::TimeoutLayer};
// use ynab_rs::{
//     apis::{self, configuration::Configuration},
//     models::TransactionDetail,
// };

// fn list_transactions(db: &Db) -> impl Iterator<Item = TransactionDetail> {
//     db.scan_prefix("txn:")
//         .filter_map(|x| x.ok())
//         .map(|(_key, value)| bincode::deserialize(&value).unwrap())
// }

// async fn sync_transactions(config: &Configuration, budget_id: &str, db: &Db) -> Result<()> {
//     // Get the saved server_knowledge.
//     let server_knowledge: Option<i64> = db
//         .get("txn-server-knowledge")
//         .ok()
//         .flatten()
//         .map(|x| bincode::deserialize(&x).unwrap());

//     let response =
//         apis::transactions_api::get_transactions(config, budget_id, None, None, server_knowledge)
//             .await?;

//     // Update the server knowledge.
//     db.insert(
//         "txn-server-knowledge",
//         bincode::serialize(&response.data.server_knowledge)?,
//     )?;

//     log::info!("🔄 Syncing transactions...");

//     for txn in response.data.transactions {
//         log::info!("🔄 txn:{}", txn.id);
//         let key = format!("txn:{}", txn.id);
//         db.insert(key, bincode::serialize(&txn)?)?;
//     }

//     log::info!("✅ Syncing transactions complete.");

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     env_logger::init();

//     dotenv()?;

//     let db = sled::open("db").expect("⚠️ Failed to open database");
//     let ynab_access_token = env::var("YNAB_ACCESS_TOKEN")?;
//     let ynab_budget_id = env::var("YNAB_BUDGET_ID")?;

//     tokio::spawn({
//         let db = db.clone();

//         async move {
//             let mut config = Configuration::new();
//             config.bearer_access_token = Some(ynab_access_token);

//             loop {
//                 sync_transactions(&config, &ynab_budget_id, &db)
//                     .await
//                     .unwrap();

//                 tokio::time::sleep(Duration::from_secs(10 * 60)).await;
//             }
//         }
//     });

//     let api = Router::new().route("/transactions", get(transactions));

//     let app = Router::new()
//         .nest("/api", api)
//         .route_service("/", ServeDir::new("site/build"))
//         .layer(TimeoutLayer::new(Duration::from_secs(10)))
//         .with_state(db.clone());

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

//     axum::serve(listener, app)
//         .with_graceful_shutdown(shutdown_signal(db))
//         .await?;

//     Ok(())
// }

// async fn transactions(State(db): State<Db>) -> String {
//     let txns = list_transactions(&db).collect::<Vec<_>>();
//     serde_json::to_string(&txns).unwrap()
// }

// async fn shutdown_signal(db: Db) {
//     let ctrl_c = async {
//         signal::ctrl_c()
//             .await
//             .expect("failed to install Ctrl+C handler");
//     };

//     #[cfg(unix)]
//     let terminate = async {
//         signal::unix::signal(signal::unix::SignalKind::terminate())
//             .expect("failed to install signal handler")
//             .recv()
//             .await;
//     };

//     #[cfg(not(unix))]
//     let terminate = std::future::pending::<()>();

//     tokio::select! {
//         _ = ctrl_c => {},
//         _ = terminate => {},
//     }

//     log::info!("🔄 Flushing database before shutdown...");
//     if let Err(e) = db.flush() {
//         log::error!("⚠️ Failed to flush database: {}", e);
//     }
//     log::info!("✅ Database shutdown complete.");
// }
