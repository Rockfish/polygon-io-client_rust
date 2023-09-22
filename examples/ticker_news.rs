use polygon_io_client_rust::common::Order;
use polygon_io_client_rust::polygon_client::PolygonClient;
use polygon_io_client_rust::ticker_news::{TickerNewsRequest, TickerNewsSort};

#[tokio::main]
async fn main() {
    let poly_client = PolygonClient::new(None, None).unwrap();

    let request = TickerNewsRequest::new()
        .ticker("AAPL")
        .order(Order::Desc)
        .sort(TickerNewsSort::Published_Utc);

    println!("Request: {:#?}", request);

    let results = poly_client.get_ticker_news(&request).await;

    println!("results: {results:#?}\n");
}
