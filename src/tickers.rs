#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use time::Date;

/// https://polygon.io/docs/stocks/get_v3_reference_tickers
const TICKERS_PATH: &'static str = "/v3/reference/tickers";

#[derive(Debug, Deserialize)]
pub struct Tickers {
    /// The total number of results for this request.
    count: i32,
    /// If present, this value can be used to fetch the next page of data.
    next_url: String,
    /// A request id assigned by the server.
    request_id: String,
    /// An array of tickers that match your query.
    results: Vec<TickersResult>,
    /// The status of this request's response.
    status: String,
}

pub enum Locale {
    US,
    Global,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Locale::US => write!(f, "us"),
            Locale::Global => write!(f, "global"),
        }
    }
}

pub enum Order {
    Asc,
    Desc,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

pub enum Market {
    Stocks,
    Crypto,
    FX,
    OTC,
    Indices,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Market::Stocks => write!(f, "stocks"),
            Market::Crypto => write!(f, "crypto"),
            Market::FX => write!(f, "fx"),
            Market::OTC => write!(f, "otc"),
            Market::Indices => write!(f, "indices"),
        }
    }
}

#[allow(non_camel_case_types)]
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
        match self {
            TickersSort::Ticker => write!(f, "stocks"),
            TickersSort::Name => write!(f, "name"),
            TickersSort::Market => write!(f, "market"),
            TickersSort::Locale => write!(f, "locale"),
            TickersSort::Primary_Exchange => write!(f, "primary_exchange"),
            TickersSort::Type => write!(f, "type"),
            TickersSort::Currency_Symbol => write!(f, "currency_symbol"),
            TickersSort::Currency_Name => write!(f, "currency_name"),
            TickersSort::Base_Currency_Symbol => write!(f, "base_currency_symbol"),
            TickersSort::Base_Currency_Name => write!(f, "base_currency_name"),
            TickersSort::Cik => write!(f, "cik"),
            TickersSort::Composite_Figi => write!(f, "composite_figi"),
            TickersSort::Share_Class_Fig => write!(f, "share_class_fig"),
            TickersSort::Last_Updated_Utc => write!(f, "last_updated_utc"),
            TickersSort::Deslisted_Utc => write!(f, "deslisted_utc"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TickersResult {
    /// Whether or not the asset is actively traded. False means the asset has been delisted.
    active: bool,

    /// The CIK number for this ticker. Find more information here.
    cik: String,

    /// The composite OpenFIGI number for this ticker. Find more information here
    composite_figi: String,

    /// The name of the currency that this asset is traded with.
    currency_name: String,

    /// The last date that the asset was traded.
    delisted_utc: String,

    /// The information is accurate up to this time.
    last_updated_utc: String,

    locale: String, // enum [us, global]
    /// The Locale of the asset.
    market: String, // *enum [stocks, crypto, fx, otc, indices]
    /// The market type of the asset.

    /// The name of the asset. For stocks/equities this will be the companies registered name.
    /// For crypto/fx this will be the name of the currency or coin pair.
    name: String,

    /// The ISO code of the primary listing exchange for this asset.
    primary_exchange: String,

    /// The share Class OpenFIGI number for this ticker. Find more information here
    share_class_figi: String,

    /// The exchange symbol that this item is traded under.
    ticker: String,

    /// The type of the asset. Find the types that we support via our Ticker Types API.
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
        request
            .parameters
            .insert("active".to_string(), "true".to_string());
        request
    }

    /// Specify a ticker symbol. Defaults to empty string which queries all tickers.
    pub fn ticker(&mut self, ticker: impl Into<String>) -> &mut TickersRequest {
        self.parameters.insert("ticker".to_string(), ticker.into());
        self
    }

    pub fn ticker_gt(&mut self, ticker: impl Into<String>) -> &mut TickersRequest {
        self.parameters
            .insert("ticker.gt".to_string(), ticker.into());
        self
    }

    pub fn ticker_gte(&mut self, ticker: impl Into<String>) -> &mut TickersRequest {
        self.parameters
            .insert("ticker.gte".to_string(), ticker.into());
        self
    }

    pub fn ticker_lt(&mut self, ticker: impl Into<String>) -> &mut TickersRequest {
        self.parameters
            .insert("ticker.lt".to_string(), ticker.into());
        self
    }

    pub fn ticker_lte(&mut self, ticker: impl Into<String>) -> &mut TickersRequest {
        self.parameters
            .insert("ticker.lte".to_string(), ticker.into());
        self
    }

    /// Filter by market type. By default all markets are included.
    pub fn market(&mut self, market: Market) -> &mut TickersRequest {
        self.parameters
            .insert("market".to_string(), market.to_string());
        self
    }

    /// Specify the primary exchange of the asset in the ISO code format.
    /// Find more information about the ISO codes at the ISO org website.
    /// Defaults to empty string which queries all exchanges.
    pub fn exchange(&mut self, exchange: impl Into<String>) -> &mut TickersRequest {
        self.parameters
            .insert("exchange".to_string(), exchange.into());
        self
    }

    /// Specify the CUSIP code of the asset you want to search for. Find more information about CUSIP codes at their website.
    /// Defaults to empty string which queries all CUSIPs.
    // Note: Although you can query by CUSIP, due to legal reasons we do not return the CUSIP in the response.
    pub fn cusip(&mut self, cusip: impl Into<String>) -> &mut TickersRequest {
        self.parameters.insert("cusip".to_string(), cusip.into());
        self
    }

    /// Specify the CIK of the asset you want to search for.
    /// Find more information about CIK codes at their website.
    /// Defaults to empty string which queries all CIKs.
    pub fn cik(&mut self, cik: impl Into<String>) -> &mut TickersRequest {
        self.parameters.insert("cik".to_string(), cik.into());
        self
    }

    /// Specify a point in time to retrieve tickers available on that date. Defaults to the most recent available date.
    pub fn date(&mut self, date: Date) -> &mut TickersRequest {
        self.parameters.insert("date".to_string(), date.to_string());
        self
    }

    /// Search for terms within the ticker and/or company name.
    pub fn search(&mut self, search: impl Into<String>) -> &mut TickersRequest {
        self.parameters.insert("search".to_string(), search.into());
        self
    }

    /// Specify if the tickers returned should be actively traded on the queried date. Default is true.
    pub fn active(&mut self, active: bool) -> &mut TickersRequest {
        self.parameters
            .insert("active".to_string(), active.to_string());
        self
    }

    /// Order results based on the sort field.
    pub fn order(&mut self, order: Order) -> &mut TickersRequest {
        self.parameters
            .insert("order".to_string(), order.to_string());
        self
    }

    /// Limit the number of results returned, default is 100 and max is 1000.
    pub fn limit(&mut self, limit: u32) -> &mut TickersRequest {
        let limit = if limit > 1000 { 1000 } else { limit };
        self.parameters
            .insert("limit".to_string(), limit.to_string());
        self
    }

    /// Sort field used for ordering.
    pub fn sort(&mut self, sort: TickersSort) -> &mut TickersRequest {
        self.parameters
            .insert("sort".to_string(), sort.to_string());
        self
    }
}
