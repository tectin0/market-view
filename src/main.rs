use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new();

    let response = provider.get_latest_quotes("AAPL", "1d");

    match response.await {
        Ok(yresponse) => {
            println!("Response: {:?}", yresponse);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
