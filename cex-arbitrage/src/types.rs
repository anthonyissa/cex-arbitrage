use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponse {
    pub symbol: String,
    pub price: String,
}

#[derive(Deserialize)]
pub struct CoinbaseApiResponse {
    pub data: CoinbaseData,
}

#[derive(Deserialize)]
pub struct CoinbaseData {
    pub amount: String,
    pub base: String,
    pub currency: String,
}

pub struct PlatformApis {
    pub binance: &'static str,
    pub coinbase: &'static str,
}

pub const PLATFORM_APIS: PlatformApis = PlatformApis {
    binance: "https://api1.binance.com/api/v3/ticker/price?symbol={}",
    coinbase: "https://api.coinbase.com/v2/prices/{}/buy",
};

pub struct PlatformFees {
    pub binance: f64,
    pub coinbase: f64,
}

pub const PLATFORM_FEES: PlatformFees = PlatformFees {
    binance: 0.001,   // 0.1% fee
    coinbase: 0.0055, // 0.55% fee
};

pub struct Arbitrage {
    pub in_token: String,
    pub out_token: String,
    pub buy_platform: String,
    pub sell_platform: String,
    pub profit: f64,
}
