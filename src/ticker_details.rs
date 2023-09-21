use crate::rest_client::*;
use std::collections::HashMap;
use serde::Deserialize;
use time::Date;

/// /v3/reference/tickers/{ticker}
/// Get a single ticker supported by Polygon.io.
/// This response will have detailed information about the ticker and the company behind it.

const TICKER_DETAILS_PATH: &'static str = "/v3/reference/tickers/{ticker}";

#[derive(Debug)]
pub struct TickerDetailsRequest {
    pub path: &'static str,
    pub parameters: HashMap<String, String>,
    pub ticker: String,
}

impl TickerDetailsRequest {
    pub fn new() -> Self {
        TickerDetailsRequest {
            path: TICKER_DETAILS_PATH,
            parameters: HashMap::new(),
            ticker: String::new(),
        }
    }

    /// The ticker symbol of the asset
    pub fn ticker(mut self, ticker: impl Into<String>) -> TickerDetailsRequest {
        self.ticker = ticker.into();
        self
    }

    /// Specify a point in time to get information about the ticker available on that date.
    pub fn date(mut self, date: Date) -> TickerDetailsRequest {
        self.parameters.insert("date".to_string(), date.to_string());
        self
    }
}

impl RequestType for TickerDetailsRequest {
    fn get_url(&self) -> String {
        format!("/v3/reference/tickers/{}", self.ticker)
    }

    fn get_query(&self) -> Vec<(&String, &String)> {
        self.parameters.iter().collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct TickerDetailsResponse {
    request_id: String,
    results: TickerDetailsResult,
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerDetailsResult {
    #[serde(default = "bool::default")]
    active: bool,
    address: Option<Address>,
    branding: Option<Branding>,
    #[serde(default = "String::default")]
    cik: String,
    #[serde(default = "String::default")]
    composite_figi: String,
    #[serde(default = "String::default")]
    currency_name: String,
    #[serde(default = "String::default")]
    description: String,
    #[serde(default = "String::default")]
    homepage_url: String,
    #[serde(default = "String::default")]
    list_date: String,
    #[serde(default = "String::default")]
    locale: String,
    #[serde(default = "String::default")]
    market: String,
    #[serde(default = "f32::default")]
    market_cap: f32,
    #[serde(default = "String::default")]
    name: String,
    #[serde(default = "String::default")]
    phone_number: String,
    #[serde(default = "String::default")]
    primary_exchange: String,
    #[serde(default = "u32::default")]
    round_lot: u32,
    #[serde(default = "String::default")]
    share_class_figi: String,
    #[serde(default = "i64::default")]
    share_class_shares_outstanding: i64,
    #[serde(default = "u32::default")]
    sic_code: u32,
    #[serde(default = "String::default")]
    sic_description: String,
    #[serde(default = "String::default")]
    ticker: String,
    #[serde(default = "String::default")]
    ticker_root: String,
    #[serde(default = "u32::default")]
    total_employees: u32,
    #[serde(default = "String::default")]
    r#type: String,
    #[serde(default = "u32::default")]
    weighted_shares_outstanding: u32,
    #[serde(default = "String::default")]
    source_feed: String,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    #[serde(default = "String::default")]
    address1: String,
    #[serde(default = "String::default")]
    city: String,
    #[serde(default = "String::default")]
    state: String,
    #[serde(default = "String::default")]
    postal_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Branding {
    #[serde(default = "String::default")]
    icon_url: String,
    #[serde(default = "String::default")]
    logo_url: String,
}
