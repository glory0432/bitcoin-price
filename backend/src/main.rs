mod controllers;
mod handlers;
mod models;
mod utils;

use std::sync::Arc;

use axum::{Router, routing::get};
use controllers::{index, ws_handler};
use handlers::fetch_price_loop;
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use utils::tracing::subscribe_tracing;

use crate::models::PriceData;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<PriceData>,
    pub db: PgPool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    subscribe_tracing();
    let database_url = std::env::var("DATABASE_URL").unwrap_or("".into());
    let app_database = PgPool::connect(&database_url)
        .await
        .map_err(|e| {
            error!("💥 Error to connect to the database: {}", e);
        })
        .unwrap();
    info!("✔ Connected to the Database!");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS prices (
            id SERIAL PRIMARY KEY,
            timestamp  TIMESTAMP NOT NULL,
            price REAL NOT NULL
        )",
    )
    .execute(&app_database)
    .await
    .map_err(|e| {
        error!("💥 Failed to create the non-exisiting price table: {}", e);
    })
    .unwrap();

    let (tx, _) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        tx: tx,
        db: app_database,
    });
    let app_state_price_fetch = app_state.clone();
    tokio::spawn(async move {
        fetch_price_loop(app_state_price_fetch).await;
    });

    let app_cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);
    let app_router: Router = Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler))
        .layer(app_cors)
        .with_state(app_state.clone());

    let app_listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| {
            error!("💥 Error to bind: {}", e);
        })
        .unwrap();

    info!("🚀 Server running started...");
    axum::serve(app_listener, app_router)
        .await
        .map_err(|e| {
            error!("Error to run the server: {}", e);
        })
        .unwrap();
}
