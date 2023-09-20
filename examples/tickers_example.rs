use polygon_io_client_rust::tickers::{Market, Order, TickersRequest, TickersSort};
use polygon_io_client_rust::utils::params_to_query_string;
use time::macros::date;

fn main() {
    let mut request = TickersRequest::new();
    request
        .ticker("AAPL")
        .market(Market::Stocks)
        .exchange("XNAS")
        .date(date!(2023 - 06 - 28))
        .order(Order::Desc)
        .sort(TickersSort::Name);

    println!("Request: {:#?}", request);

    println!("Query: {}", params_to_query_string(request.parameters));
}
