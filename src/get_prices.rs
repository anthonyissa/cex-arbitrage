use futures::{executor, future};
use reqwest;
use types::BinanceApiResponse;
use types::CoinbaseApiResponse;

use crate::types;

pub fn get_platform_url(platform: &str) -> &'static str {
    match platform {
        "binance" => types::PLATFORM_APIS.binance,
        "coinbase" => types::PLATFORM_APIS.coinbase,
        _ => panic!("Platform not supported"),
    }
}

pub fn format_symbol(in_token: &str, out_token: &str, platform: &str) -> String {
    match platform {
        "binance" => format!("{}{}", in_token, out_token),
        "coinbase" => format!("{}-{}", in_token, out_token),
        _ => panic!("Platform not supported"),
    }
}

pub fn format_response(response: String, platform: &str) -> f64 {
    match platform {
        "binance" => {
            let binance_response: BinanceApiResponse = serde_json::from_str(&response).unwrap();
            binance_response.price.parse::<f64>().unwrap()
        }
        "coinbase" => {
            let coinbase_response: CoinbaseApiResponse = serde_json::from_str(&response).unwrap();
            coinbase_response.data.amount.parse::<f64>().unwrap()
        }
        _ => panic!("Platform not supported"),
    }
}

pub async fn get_price(
    in_token: &str,
    out_token: &str,
    platform: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let formatted_symbol: String = format_symbol(in_token, out_token, platform);
    let url: String = get_platform_url(platform).replace("{}", &formatted_symbol);
    let response: String = reqwest::get(url).await?.text().await?;
    let price: f64 = format_response(response, platform);
    Ok(price)
}

pub async fn get_prices(
    in_token: &str,
    out_token: &str,
    platforms: Vec<&str>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let futures = future::join_all(
        platforms
            .iter()
            .map(|platform| get_price(in_token, out_token, platform))
            .collect::<Vec<_>>(),
    );
    let prices: Vec<f64> = futures
        .await
        .into_iter()
        .map(|result| result.unwrap())
        .collect();
    Ok(prices)
}
