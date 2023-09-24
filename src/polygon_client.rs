use crate::dividends::{DividendRequest, DividendsResponse};
use crate::rest_client::RestClient;
use crate::ticker_details::{TickerDetailsRequest, TickerDetailsResponse};
use crate::ticker_news::{TickerNewsRequest, TickerNewsResponse};
use crate::tickers::{TickersRequest, TickersResponse};
use std::env;

static POLYGON_IO_API_URL: &str = "https://api.polygon.io";

pub struct PolygonClient {
    rest_client: RestClient,
}

pub struct PolygonClientBuilder {
    auth_key: Option<String>,
    timeout: Option<core::time::Duration>,
    error: Option<String>,
    // rate_limit: u32
}

impl PolygonClientBuilder {
    pub fn new() -> Self {
        PolygonClientBuilder {
            auth_key: None,
            timeout: Default::default(),
            error: None,
        }
    }

    pub fn auth_key(mut self, auth_key: impl Into<String>) -> PolygonClientBuilder {
        self.auth_key = Some(auth_key.into());
        self
    }

    pub fn auth_key_env(mut self, env_var: &str) -> PolygonClientBuilder {
        match env::var(env_var) {
            Ok(val) => self.auth_key = Some(val),
            _ => self.error = Some(format!("Auth Key not found. Environment variable '{}' is not set", env_var)),
        };
        self
    }

    pub fn timeout(mut self, timeout: core::time::Duration) -> PolygonClientBuilder {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<PolygonClient, String> {
        if let Some(err) = self.error {
            return Err(err);
        }

        let api_url = match env::var("POLYGON_API_URL") {
            Ok(v) => v,
            _ => String::from(POLYGON_IO_API_URL),
        };

        match self.auth_key {
            None => Err("Auth key is not set".to_string()),
            Some(auth_key) => {
                let rest_client = RestClient::new(api_url, auth_key, self.timeout).unwrap();
                Ok(PolygonClient { rest_client })
            }
        }
    }
}

impl Default for PolygonClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PolygonClient {
    /// Query all ticker symbols which are supported by Polygon.io.
    /// This API currently includes Stocks/Equities, Indices, Forex, and Crypto.
    /// [/v3/reference/tickers](https://polygon.io/docs/stocks/get_v3_reference_tickers)
    pub async fn get_tickers(&self, request: &TickersRequest) -> Result<TickersResponse, reqwest::Error> {
        self.rest_client.send_request::<TickersResponse>(request).await
    }

    /// Get a single ticker supported by Polygon.io.
    /// This response will have detailed information about the ticker and the company behind it.
    /// [/v3/reference/tickers/{ticker}](https://polygon.io/docs/stocks/get_v3_reference_tickers__ticker)
    // pub async fn get_tickers_details(&self, request: &TickerDetailsRequest) -> Result<TickerDetailsResponse, reqwest::Error> {
    pub async fn get_tickers_details(&self, request: &TickerDetailsRequest) -> Result<TickerDetailsResponse, reqwest::Error> {
        self.rest_client.send_request::<TickerDetailsResponse>(request).await
    }

    /// Get the most recent news articles relating to a stock ticker symbol,
    /// including a summary of the article and a link to the original source.
    /// [/v2/reference/news](https://polygon.io/docs/stocks/get_v2_reference_news)
    pub async fn get_ticker_news(&self, request: &TickerNewsRequest) -> Result<TickerNewsResponse, reqwest::Error> {
        self.rest_client.send_request::<TickerNewsResponse>(request).await
    }

    // Get a list of historical cash dividends, including the ticker symbol, declaration date, ex-dividend date, record date, pay date, frequency, and amount.
    /// [/v3/reference/dividends](https://polygon.io/docs/stocks/get_v3_reference_dividends)
    pub async fn get_dividends(&self, request: &DividendRequest) -> Result<DividendsResponse, reqwest::Error> {
        self.rest_client.send_request::<DividendsResponse>(request).await
    }
}
