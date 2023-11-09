mod binance;
mod types;
use binance::get_binance_price;

#[tokio::main]
async fn main() {
    match get_binance_price("ETHUSDT").await {
        Ok(price) => println!("Binance price: {}", price),
        Err(e) => eprintln!("Error getting Binance price: {:?}", e),
    };
}
