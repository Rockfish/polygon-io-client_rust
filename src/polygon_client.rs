use crate::rest_client::{RequestType, RestClient};
use crate::ticker_details::{TickerDetailsRequest, TickerDetailsResponse};
use crate::tickers::{TickersRequest, TickersResponse};
use std::env;

static POLYGON_IO_API_URL: &str = "https://api.polygon.io";

pub struct PolygonClient {
    client: RestClient,
}

impl PolygonClient {

    pub fn new(auth_key: Option<&str>, timeout: Option<core::time::Duration>) -> Result<Self, String> {
        let api_url = match env::var("POLYGON_API_URL") {
            Ok(v) => v,
            _ => String::from(POLYGON_IO_API_URL),
        };

        let auth_key_actual = match auth_key {
            Some(v) => String::from(v),
            _ => match env::var("POLYGON_AUTH_KEY") {
                Ok(v) => v,
                _ => return Err("POLYGON_AUTH_KEY not set".to_string()),
            },
        };

        let rest_client = RestClient::new(api_url, auth_key_actual, timeout).unwrap();

        Ok(PolygonClient {
            client: rest_client,
        })
    }

    /// Query all ticker symbols which are supported by Polygon.io.
    /// This API currently includes Stocks/Equities, Indices, Forex, and Crypto.
    /// [/v3/reference/tickers](https://polygon.io/docs/stocks/get_v3_reference_tickers)
    pub async fn get_tickers(&self, request: &TickersRequest) -> Result<TickersResponse, reqwest::Error> {
        self.client.send_request::<TickersResponse>(request).await
    }

    /// Get a single ticker supported by Polygon.io.
    /// This response will have detailed information about the ticker and the company behind it.
    /// [/v3/reference/tickers/{ticker}](https://polygon.io/docs/stocks/get_v3_reference_tickers__ticker)
    // pub async fn get_tickers_details(&self, request: &TickerDetailsRequest) -> Result<TickerDetailsResponse, reqwest::Error> {
        pub async fn get_tickers_details(&self, request: &TickerDetailsRequest) -> Result<TickerDetailsResponse, reqwest::Error> {
        self.client.send_request::<TickerDetailsResponse>(request).await
    }
}
