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
