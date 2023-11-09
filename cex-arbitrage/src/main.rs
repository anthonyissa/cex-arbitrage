mod get_prices;
mod types;
use get_prices::get_prices;

#[tokio::main]
async fn main() {
    match get_prices("ETH", "USDT", ["binance", "coinbase"].to_vec()).await {
        Ok(price) => println!("Platform prices: {:?}", price),
        Err(e) => eprintln!("Error getting platform prices: {:?}", e),
    };
}
