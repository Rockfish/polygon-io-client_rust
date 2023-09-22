use crate::common::Order;
use crate::rest_client::RequestType;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DividendsResponse {
    #[serde(default = "String::default")]
    next_url: String,
    request_id: String,
    results: Vec<Dividend>,
    status: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Dividend {
    #[serde(default = "f32::default")]
    cash_amount: f32,
    #[serde(default = "String::default")]
    declaration_date: String,
    dividend_type: DividendType,
    #[serde(default = "String::default")]
    ex_dividend_date: String,
    #[serde(default = "i32::default")]
    frequency: i32,
    #[serde(default = "String::default")]
    pay_date: String,
    #[serde(default = "String::default")]
    record_date: String,
    #[serde(default = "String::default")]
    ticker: String,
}

#[derive(Debug, Deserialize)]
pub enum DividendType {
    CD,
    SC,
    LT,
    ST,
}

impl fmt::Display for DividendType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum DividendSort {
    Ex_Dividend_Date,
    Pay_Date,
    Declaration_Date,
    Record_Date,
    Cash_Amount,
    Ticker,
}

impl fmt::Display for DividendSort {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum DividendFrequency {
    one_time = 0,
    annually = 1,
    bi_annually = 2,
    quarterly = 4,
    monthly = 12,
}

impl fmt::Display for DividendFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = *self as u32;
        write!(f, "{}", val)
    }
}

#[derive(Debug)]
pub struct DividendRequest {
    pub parameters: HashMap<String, String>,
}

impl RequestType for DividendRequest {
    fn get_url(&self) -> String {
        "/v3/reference/dividends".to_string()
    }

    fn get_query(&self) -> Vec<(&String, &String)> {
        self.parameters.iter().collect()
    }
}

impl DividendRequest {
    pub fn new() -> Self {
        DividendRequest {
            parameters: Default::default(),
        }
    }

    /// Specify a ticker symbol. Defaults to empty string which queries all tickers.
    pub fn ticker(mut self, ticker: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ticker".to_string(), ticker.into());
        self
    }

    pub fn ticker_gt(mut self, ticker: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ticker.gt".to_string(), ticker.into());
        self
    }

    pub fn ticker_gte(mut self, ticker: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ticker.gte".to_string(), ticker.into());
        self
    }

    pub fn ticker_lt(mut self, ticker: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ticker.lt".to_string(), ticker.into());
        self
    }

    pub fn ticker_lte(mut self, ticker: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ticker.lte".to_string(), ticker.into());
        self
    }

    pub fn ex_dividend_date(mut self, ex_dividend_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ex_dividend_date".to_string(), ex_dividend_date.into());
        self
    }

    pub fn ex_dividend_date_gt(mut self, ex_dividend_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ex_dividend_date.gt".to_string(), ex_dividend_date.into());
        self
    }

    pub fn ex_dividend_date_gte(mut self, ex_dividend_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ex_dividend_date.gte".to_string(), ex_dividend_date.into());
        self
    }

    pub fn ex_dividend_date_lt(mut self, ex_dividend_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ex_dividend_date.lt".to_string(), ex_dividend_date.into());
        self
    }

    pub fn ex_dividend_date_lte(mut self, ex_dividend_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("ex_dividend_date.lte".to_string(), ex_dividend_date.into());
        self
    }

    pub fn record_date(mut self, record_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("record_date".to_string(), record_date.into());
        self
    }

    pub fn record_date_gt(mut self, record_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("record_date.gt".to_string(), record_date.into());
        self
    }

    pub fn record_date_gte(mut self, record_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("record_date.gte".to_string(), record_date.into());
        self
    }

    pub fn record_date_lt(mut self, record_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("record_date.lt".to_string(), record_date.into());
        self
    }

    pub fn record_date_lte(mut self, record_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("record_date.lte".to_string(), record_date.into());
        self
    }

    pub fn declaration_date(mut self, declaration_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("declaration_date".to_string(), declaration_date.into());
        self
    }

    pub fn declaration_date_gt(mut self, declaration_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("declaration_date.gt".to_string(), declaration_date.into());
        self
    }

    pub fn declaration_date_gte(mut self, declaration_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("declaration_date.gte".to_string(), declaration_date.into());
        self
    }

    pub fn declaration_date_lt(mut self, declaration_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("declaration_date.lt".to_string(), declaration_date.into());
        self
    }

    pub fn declaration_date_lte(mut self, declaration_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("declaration_date.lte".to_string(), declaration_date.into());
        self
    }

    pub fn pay_date(mut self, pay_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("pay_date".to_string(), pay_date.into());
        self
    }

    pub fn pay_date_gt(mut self, pay_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("pay_date.gt".to_string(), pay_date.into());
        self
    }

    pub fn pay_date_gte(mut self, pay_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("pay_date.gte".to_string(), pay_date.into());
        self
    }

    pub fn pay_date_lt(mut self, pay_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("pay_date.lt".to_string(), pay_date.into());
        self
    }

    pub fn pay_date_lte(mut self, pay_date: impl Into<String>) -> DividendRequest {
        self.parameters.insert("pay_date.lte".to_string(), pay_date.into());
        self
    }

    pub fn cash_amount(mut self, cash_amount: impl Into<String>) -> DividendRequest {
        self.parameters.insert("cash_amount".to_string(), cash_amount.into());
        self
    }

    pub fn cash_amount_gt(mut self, cash_amount: impl Into<String>) -> DividendRequest {
        self.parameters.insert("cash_amount.gt".to_string(), cash_amount.into());
        self
    }

    pub fn cash_amount_gte(mut self, cash_amount: impl Into<String>) -> DividendRequest {
        self.parameters.insert("cash_amount.gte".to_string(), cash_amount.into());
        self
    }

    pub fn cash_amount_lt(mut self, cash_amount: impl Into<String>) -> DividendRequest {
        self.parameters.insert("cash_amount.lt".to_string(), cash_amount.into());
        self
    }

    pub fn cash_amount_lte(mut self, cash_amount: impl Into<String>) -> DividendRequest {
        self.parameters.insert("cash_amount.lte".to_string(), cash_amount.into());
        self
    }

    pub fn frequency(mut self, frequency: DividendFrequency) -> DividendRequest {
        self.parameters.insert("frequency".to_string(), frequency.to_string());
        self
    }

    pub fn dividend_type(mut self, dividend_type: DividendType) -> DividendRequest {
        self.parameters.insert("dividend_type".to_string(), dividend_type.to_string());
        self
    }

    pub fn order(mut self, order: Order) -> DividendRequest {
        self.parameters.insert("order".to_string(), order.to_string());
        self
    }

    /// Limit the number of results returned, default is 100 and max is 1000.
    pub fn limit(mut self, limit: u32) -> DividendRequest {
        let limit = if limit > 1000 { 1000 } else { limit };
        self.parameters.insert("limit".to_string(), limit.to_string());
        self
    }

    /// Sort field used for ordering.
    pub fn sort(mut self, sort: DividendSort) -> DividendRequest {
        self.parameters.insert("sort".to_string(), sort.to_string());
        self
    }
}
