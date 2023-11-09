use reqwest;
use types::ApiResponse;

use crate::types;

pub async fn get_binance_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api1.binance.com/api/v3/ticker/price?symbol={}",
        symbol
    );
    let response = reqwest::get(url).await?.text().await?;
    let json: types::ApiResponse = serde_json::from_str(&response)?;
    let price: f64 = json.price.parse()?;
    Ok(price)
}
