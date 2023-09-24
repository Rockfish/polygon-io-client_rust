#![allow(dead_code)]

pub use crate::common::{Market, Order};
use crate::rest_client::RequestType;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use time::Date;

/// https://polygon.io/docs/stocks/get_v3_reference_tickers
const TICKERS_PATH: &str = "/v3/reference/tickers";

#[derive(Debug, Deserialize)]
pub struct TickersResponse {
    /// The total number of results for this request.
    count: i32,
    /// If present, this value can be used to fetch the next page of data.
    #[serde(default = "String::default")]
    next_url: String,
    /// A request id assigned by the server.
    #[serde(default = "String::default")]
    request_id: String,
    /// An array of tickers that match your query.
    #[serde(default = "Vec::default")]
    results: Vec<TickersResult>,
    /// The status of this request's response.
    #[serde(default = "String::default")]
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct TickersResult {
    /// Whether or not the asset is actively traded. False means the asset has been delisted.
    #[serde(default = "bool::default")]
    active: bool,

    /// The CIK number for this ticker. Find more information here.
    #[serde(default = "String::default")]
    cik: String,

    /// The composite OpenFIGI number for this ticker. Find more information here
    #[serde(default = "String::default")]
    composite_figi: String,

    /// The name of the currency that this asset is traded with.
    #[serde(default = "String::default")]
    currency_name: String,

    /// The last date that the asset was traded.
    #[serde(default = "String::default")]
    delisted_utc: String,

    /// The information is accurate up to this time.
    #[serde(default = "String::default")]
    last_updated_utc: String,

    /// The Locale of the asset.
    #[serde(default = "String::default")]
    locale: String, // enum [us, global]

    /// The market type of the asset.
    #[serde(default = "String::default")]
    market: String, // *enum [stocks, crypto, fx, otc, indices]

    /// The name of the asset. For stocks/equities this will be the companies registered name.
    /// For crypto/fx this will be the name of the currency or coin pair.
    #[serde(default = "String::default")]
    name: String,

    /// The ISO code of the primary listing exchange for this asset.
    #[serde(default = "String::default")]
    primary_exchange: String,

    /// The share Class OpenFIGI number for this ticker. Find more information here
    #[serde(default = "String::default")]
    share_class_figi: String,

    /// The exchange symbol that this item is traded under.
    #[serde(default = "String::default")]
    ticker: String,

    /// The type of the asset. Find the types that we support via our Ticker Types API.
    #[serde(default = "String::default")]
    r#type: String,
}

// scheme://host:port/path?queryString#fragment
#[derive(Debug)]
pub struct TickersRequest {
    pub path: &'static str,
    pub parameters: HashMap<String, String>,
}

impl Default for TickersRequest {
    fn default() -> Self {
        TickersRequest {
            path: TICKERS_PATH,
            parameters: HashMap::new(),
        }
    }
}

impl TickersRequest {
    pub fn new() -> Self {
        let mut request = TickersRequest::default();
        request.parameters.insert("active".to_string(), "true".to_string());
        request
    }

    /// Specify a ticker symbol. Defaults to empty string which queries all tickers.
    pub fn ticker(mut self, ticker: impl Into<String>) -> TickersRequest {
        self.parameters.insert("ticker".to_string(), ticker.into());
        self
    }

    pub fn ticker_gt(mut self, ticker: impl Into<String>) -> TickersRequest {
        self.parameters.insert("ticker.gt".to_string(), ticker.into());
        self
    }

    pub fn ticker_gte(mut self, ticker: impl Into<String>) -> TickersRequest {
        self.parameters.insert("ticker.gte".to_string(), ticker.into());
        self
    }

    pub fn ticker_lt(mut self, ticker: impl Into<String>) -> TickersRequest {
        self.parameters.insert("ticker.lt".to_string(), ticker.into());
        self
    }

    pub fn ticker_lte(mut self, ticker: impl Into<String>) -> TickersRequest {
        self.parameters.insert("ticker.lte".to_string(), ticker.into());
        self
    }

    /// Filter by market type. By default all markets are included.
    pub fn market(mut self, market: Market) -> TickersRequest {
        self.parameters.insert("market".to_string(), market.to_string());
        self
    }

    /// Specify the primary exchange of the asset in the ISO code format.
    /// Find more information about the ISO codes at the ISO org website.
    /// Defaults to empty string which queries all exchanges.
    pub fn exchange(mut self, exchange: impl Into<String>) -> TickersRequest {
        self.parameters.insert("exchange".to_string(), exchange.into());
        self
    }

    /// Specify the CUSIP code of the asset you want to search for. Find more information about CUSIP codes at their website.
    /// Defaults to empty string which queries all CUSIPs.
    // Note: Although you can query by CUSIP, due to legal reasons we do not return the CUSIP in the response.
    pub fn cusip(mut self, cusip: impl Into<String>) -> TickersRequest {
        self.parameters.insert("cusip".to_string(), cusip.into());
        self
    }

    /// Specify the CIK of the asset you want to search for.
    /// Find more information about CIK codes at their website.
    /// Defaults to empty string which queries all CIKs.
    pub fn cik(mut self, cik: impl Into<String>) -> TickersRequest {
        self.parameters.insert("cik".to_string(), cik.into());
        self
    }

    /// Specify a point in time to retrieve tickers available on that date. Defaults to the most recent available date.
    pub fn date(mut self, date: Date) -> TickersRequest {
        self.parameters.insert("date".to_string(), date.to_string());
        self
    }

    /// Search for terms within the ticker and/or company name.
    pub fn search(mut self, search: impl Into<String>) -> TickersRequest {
        self.parameters.insert("search".to_string(), search.into());
        self
    }

    /// Specify if the tickers returned should be actively traded on the queried date. Default is true.
    pub fn active(mut self, active: bool) -> TickersRequest {
        self.parameters.insert("active".to_string(), active.to_string());
        self
    }

    /// Order results based on the sort field.
    pub fn order(mut self, order: Order) -> TickersRequest {
        self.parameters.insert("order".to_string(), order.to_string());
        self
    }

    /// Limit the number of results returned, default is 100 and max is 1000.
    pub fn limit(mut self, limit: u32) -> TickersRequest {
        let limit = if limit > 1000 { 1000 } else { limit };
        self.parameters.insert("limit".to_string(), limit.to_string());
        self
    }

    /// Sort field used for ordering.
    pub fn sort(mut self, sort: TickersSort) -> TickersRequest {
        self.parameters.insert("sort".to_string(), sort.to_string());
        self
    }
}

impl RequestType for TickersRequest {
    fn get_url(&self) -> String {
        self.path.to_string()
    }

    fn get_query(&self) -> Vec<(&String, &String)> {
        self.parameters.iter().collect()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TickersSort {
    Ticker,
    Name,
    Market,
    Locale,
    Primary_Exchange,
    Type,
    Currency_Symbol,
    Currency_Name,
    Base_Currency_Symbol,
    Base_Currency_Name,
    Cik,
    Composite_Figi,
    Share_Class_Fig,
    Last_Updated_Utc,
    Deslisted_Utc,
}

impl fmt::Display for TickersSort {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}
