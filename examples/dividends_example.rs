use polygon_io_client_rust::common::Order;
use polygon_io_client_rust::dividends::{DividendFrequency, DividendRequest, DividendSort};
use polygon_io_client_rust::polygon_client::PolygonClient;

#[tokio::main]
async fn main() {
    let poly_client = PolygonClient::new(None, None).unwrap();

    let request = DividendRequest::new()
        .ticker("AAPL")
        .order(Order::Desc)
        .frequency(DividendFrequency::quarterly)
        .sort(DividendSort::Cash_Amount);

    println!("Request: {:#?}", request);

    let results = poly_client.get_dividends(&request).await;

    println!("results: {results:#?}\n");
}
