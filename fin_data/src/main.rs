use serde::Deserialize;
use std::error::Error;

pub trait Pricing {
    fn fetch_price(&self) -> ApiResult<PriceData>;
    fn save_to_file(&self, data: &PriceData);
}


#[derive(Debug, Deserialize)]
pub struct PriceData {
    pub bitcoin: Option<CurrencyData>,
    pub ethereum: Option<CurrencyData>,
    pub sp500: Option<CurrencyData>,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyData {
    pub usd: f64,
}

pub enum ApiResult<T> {
    Success(T),
    ApiError(String),
    NetworkError(String),
}

pub struct Bitcoin;
pub struct Ethereum;
pub struct SP500;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> ApiResult<PriceData> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<PriceData>() {
                        Ok(f_data) => ApiResult::Success(f_data),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, data: &PriceData) {
        if let Some(bitcoin_data) = &data.bitcoin {
            let _ = std::fs::write("bitcoin_price.txt", format!("Bitcoin price: ${}", bitcoin_data.usd));
        }
    }
}


impl Pricing for Ethereum {
    fn fetch_price(&self) -> ApiResult<PriceData> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<PriceData>() {
                        Ok(f_data) => ApiResult::Success(f_data),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, data: &PriceData) {
        if let Some(ethereum_data) = &data.ethereum {
            let _ = std::fs::write("ethereum_price.txt", format!("Ethereum price: ${}", ethereum_data.usd));
        }
    }
}

// Get S&P 500 data using Yahoo Finance
impl Pricing for SP500 {
    fn fetch_price(&self) -> ApiResult<PriceData> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/^GSPC?range=1d&interval=1m";
        
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<PriceData>() {
                        Ok(f_data) => ApiResult::Success(f_data),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, data: &PriceData) {
        if let Some(sp500_data) = &data.sp500 {
            let _ = std::fs::write("SP500_price.txt", format!("SP500 price: ${}", sp500_data.usd));
        }
    }
}


fn main() {
    // Initialization: Instantiate the structs and store them in a vector.
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    // Data Fetching Loop: 
    // Iterate over each asset
    // Fetch and save the latest pricing data.
    // Pause for 10 seconds.
    loop {
        for asset in &assets {
            match asset.fetch_price() {
                ApiResult::Success(data) => {
                    println!("Fetched data: {:?}", data);
                    asset.save_to_file(&data);
                }
                ApiResult::ApiError(e) => eprintln!("API error: {}", e),
                ApiResult::NetworkError(e) => eprintln!("Network error: {}", e),
            }
        }

        println!("Waiting 10 seconds before next fetch...");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}