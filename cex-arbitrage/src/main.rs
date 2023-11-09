mod arbitrage;
mod get_prices;
mod types;
use arbitrage::check_arbitrage_opportunity;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    loop {
        check_arbitrage_opportunity(["binance", "coinbase"].to_vec()).await;
        sleep(Duration::from_secs(10)).await;
    }
}
