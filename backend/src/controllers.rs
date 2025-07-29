use std::sync::Arc;

use crate::{models::PriceData, AppState};
use chrono::{NaiveDateTime, TimeZone, Utc};
use sqlx::Row;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::stream::StreamExt;
use lazy_static::lazy_static;
use tokio::select;
use tracing::{info, error};
lazy_static! {
    static ref SENDER_BOUND: usize = 500;
}

pub async fn index() -> impl IntoResponse {
    "Bitcoin Price"
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();
    match sqlx::query("SELECT price, timestamp FROM prices").fetch_all(&state.db).await {
        Ok(rows) => {
            for row in rows {
                
                let timestamp : NaiveDateTime = row.get("timestamp");
                let price: f32= row.get("price");
                let timestamp = Utc.from_utc_datetime(&timestamp);
                let price_data = PriceData{
                    timestamp: timestamp.to_rfc3339(),
                    price: price as f64
                };
                let json = serde_json::to_string(&price_data).unwrap();
                let _ = socket.send(Message::Text(json.into())).await;
            }
        },
        Err(e) => {
            error!("Failed to send stored data in db: {}", e);
        }
    }
    loop {
        
        select! {
            result = rx.recv() => {
                match result {
                    Ok(price_update) => {
                        let json = serde_json::to_string(&price_update).unwrap();
                        if socket.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    },
                    Err(e) => {
                        error!("Broadcast receive error: {:?}", e);
                        break;
                    }
                }
            }
            msg = socket.next() => {
                if let Some(Ok(msg)) = msg {
                    match msg {
                        Message::Close(_) => break,
                        _ => {}
                    }
                } else {
                    break;
                }
            }
        }
    }
    info!("Websocket disconnected ");
}
