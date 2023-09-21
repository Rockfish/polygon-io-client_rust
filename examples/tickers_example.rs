use polygon_io_client_rust::polygon_client::*;
use polygon_io_client_rust::tickers::{Market, Order, TickersRequest, TickersSort};
use polygon_io_client_rust::utils::params_to_query_string;
use time::macros::date;
use polygon_io_client_rust::ticker_details::TickerDetailsRequest;

#[tokio::main]
async fn main() {
    let poly_client = PolygonClient::new(None, None).unwrap();

    let mut request = TickersRequest::new()
        .ticker("AAPL")
        .market(Market::Stocks)
        .exchange("XNAS")
        .date(date!(2023 - 06 - 28))
        .order(Order::Desc)
        .sort(TickersSort::Name);

    println!("Request: {:#?}", request);
    println!("Query: {}", params_to_query_string(&request.parameters));

    let results = poly_client.get_tickers(&request).await;
    println!("results: {results:#?}\n");


    let request = TickerDetailsRequest::new().ticker("BP").date(date!(2023 - 06 - 28));

    println!("Request: {:#?}", request);
    let results = poly_client.get_tickers_details(&request).await;
    println!("results: {results:#?}\n");
}
