use serde::Serialize;
#[derive(Serialize, Clone, Debug)]
pub struct PriceData {
    pub timestamp: String,
    pub price: f64,
}
