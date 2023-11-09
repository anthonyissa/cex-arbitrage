use get_prices::get_prices;

use crate::get_prices;
use crate::notification::send_notification;
use crate::types;

fn get_min_max_prices(prices: Vec<f64>, platforms: Vec<&str>) -> (f64, f64, String, String) {
    let mut min_price: f64 = prices[0];
    let mut max_price: f64 = prices[0];
    let mut min_platform: String = platforms[0].to_string();
    let mut max_platform: String = platforms[0].to_string();
    for (i, price) in prices.iter().enumerate() {
        if price < &min_price {
            min_price = *price;
            min_platform = platforms[i].to_string();
        }
        if price > &max_price {
            max_price = *price;
            max_platform = platforms[i].to_string();
        }
    }
    (min_price, max_price, min_platform, max_platform)
}

fn calculate_profit(
    min_price: f64,
    max_price: f64,
    min_platform: String,
    max_platform: String,
) -> f64 {
    let min_fee: f64 = match min_platform.as_str() {
        "binance" => types::PLATFORM_FEES.binance,
        "coinbase" => types::PLATFORM_FEES.coinbase,
        _ => panic!("Platform not supported"),
    };
    let max_fee: f64 = match max_platform.as_str() {
        "binance" => types::PLATFORM_FEES.binance,
        "coinbase" => types::PLATFORM_FEES.coinbase,
        _ => panic!("Platform not supported"),
    };
    let profit: f64 = (max_price * (1.0 - max_fee)) - (min_price * (1.0 + min_fee));
    profit
}

pub async fn check_arbitrage_opportunity(platforms: Vec<&str>) {
    for token in types::TOKENS.iter() {
        println!("Checking {}...", token);
        let prices = get_prices(token, "USDT", platforms.clone()).await.unwrap();
        let (min_price, max_price, min_platform, max_platform) =
            get_min_max_prices(prices, platforms.clone());
        let profit: f64 = calculate_profit(
            min_price,
            max_price,
            min_platform.clone(),
            max_platform.clone(),
        );
        println!("Min price: ${} on {}", min_price, min_platform);
        println!("Max price: ${} on {}", max_price, max_platform);
        println!("Profit: ${}", profit);
        if profit > 0.0 {
            let notification_text: String = format!(
                "Buy {} on {} for ${} and sell on {} for ${} for a profit of ${}",
                token, min_platform, min_price, max_platform, max_price, profit
            );
            println!("{}", notification_text);
            send_notification(&notification_text).await;
        }
    }
}
