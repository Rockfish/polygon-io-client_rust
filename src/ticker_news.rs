use crate::common::Order;
use crate::rest_client::RequestType;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TickerNewsResponse {
    status: String,
    request_id: String,
    count: i32,
    #[serde(default = "String::default")]
    previous_url: String,
    #[serde(default = "String::default")]
    next_url: String,
    #[serde(default = "Vec::default")]
    results: Vec<TickerNewsResults>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TickerNewsResults {
    #[serde(default = "String::default")]
    amp_url: String,
    #[serde(default = "String::default")]
    article_url: String,
    #[serde(default = "String::default")]
    author: String,
    #[serde(default = "String::default")]
    description: String,
    #[serde(default = "String::default")]
    id: String,
    #[serde(default = "String::default")]
    image_url: String,
    #[serde(default = "Vec::default")]
    keywords: Vec<String>,
    #[serde(default = "String::default")]
    published_utc: String,
    publisher: Option<Publisher>,
    #[serde(default = "Vec::default")]
    tickers: Vec<String>,
    #[serde(default = "String::default")]
    title: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Publisher {
    #[serde(default = "String::default")]
    favicon_url: String,
    #[serde(default = "String::default")]
    homepage_url: String,
    #[serde(default = "String::default")]
    logo_url: String,
    #[serde(default = "String::default")]
    name: String,
}

#[derive(Debug)]
pub struct TickerNewsRequest {
    pub parameters: HashMap<String, String>,
}

impl RequestType for TickerNewsRequest {
    fn get_url(&self) -> String {
        "/v2/reference/news".to_string()
    }

    fn get_query(&self) -> Vec<(&String, &String)> {
        self.parameters.iter().collect()
    }
}

impl TickerNewsRequest {
    pub fn new() -> Self {
        TickerNewsRequest {
            parameters: Default::default(),
        }
    }

    /// Specify a ticker symbol. Defaults to empty string which queries all tickers.
    pub fn ticker(mut self, ticker: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("ticker".to_string(), ticker.into());
        self
    }

    pub fn ticker_gt(mut self, ticker: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("ticker.gt".to_string(), ticker.into());
        self
    }

    pub fn ticker_gte(mut self, ticker: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("ticker.gte".to_string(), ticker.into());
        self
    }

    pub fn ticker_lt(mut self, ticker: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("ticker.lt".to_string(), ticker.into());
        self
    }

    pub fn ticker_lte(mut self, ticker: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("ticker.lte".to_string(), ticker.into());
        self
    }

    pub fn published_utc(mut self, published_utc: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("published_utc".to_string(), published_utc.into());
        self
    }

    pub fn published_utc_gt(mut self, published_utc: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("published_utc.gt".to_string(), published_utc.into());
        self
    }

    pub fn published_utc_gte(mut self, published_utc: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("published_utc.gte".to_string(), published_utc.into());
        self
    }

    pub fn published_utc_lt(mut self, published_utc: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("published_utc.lt".to_string(), published_utc.into());
        self
    }

    pub fn published_utc_lte(mut self, published_utc: impl Into<String>) -> TickerNewsRequest {
        self.parameters.insert("published_utc.lte".to_string(), published_utc.into());
        self
    }

    /// Order results based on the sort field.
    pub fn order(mut self, order: Order) -> TickerNewsRequest {
        self.parameters.insert("order".to_string(), order.to_string());
        self
    }

    /// Limit the number of results returned, default is 100 and max is 1000.
    pub fn limit(mut self, limit: u32) -> TickerNewsRequest {
        let limit = if limit > 1000 { 1000 } else { limit };
        self.parameters.insert("limit".to_string(), limit.to_string());
        self
    }

    /// Sort field used for ordering.
    pub fn sort(mut self, sort: TickerNewsSort) -> TickerNewsRequest {
        self.parameters.insert("sort".to_string(), sort.to_string());
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TickerNewsSort {
    Published_Utc,
}

impl fmt::Display for TickerNewsSort {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}
