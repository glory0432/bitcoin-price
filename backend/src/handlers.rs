use std::sync::Arc;

use chrono::Utc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

use crate::{AppState, models::PriceData};

pub async fn fetch_bitcoin_price() -> Result<f64, anyhow::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json_response: serde_json::Value = resp.json().await?;
        if let Some(price) = json_response
            .get("bitcoin")
            .and_then(|b| b.get("usd"))
            .and_then(|v| v.as_f64())
        {
            Ok(price)
        } else {
            Err(anyhow::anyhow!("Unexpected JSON response"))
        }
    } else {
        Err(anyhow::anyhow!("HTTP error: {}", resp.status()))
    }
}
pub async fn fetch_price_loop(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(15));

    loop {
        interval.tick().await;

        match fetch_bitcoin_price().await {
            Ok(price) => {
                let now = Utc::now();
                let timestamp = now.to_rfc3339();
                info!("Bitcoin price fetched: {} USD at {}", timestamp, price);

                if let Err(e) = sqlx::query("INSERT INTO prices (timestamp, price) VALUES ($1, $2)")
                    .bind(&now)
                    .bind(price)
                    .execute(&state.db)
                    .await
                {
                    error!("Database error: {:?}", e);
                }

                let price_data = PriceData {
                    timestamp: timestamp.clone(),
                    price: price,
                };
                match state.tx.send(price_data.clone()) {
                    Ok(_) => {
                        info!("Broadcasted: {} USD at {}", price, timestamp);
                    }
                    Err(e) => {
                        error!("Broadcast error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch bitcoin price: {:?}", e);
            }
        }
    }
}
