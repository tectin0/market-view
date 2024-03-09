use std::ops::Add;
use std::sync::{Arc, Mutex};


use yahoo_finance_api::time::Duration;
use yahoo_finance_api::{time::OffsetDateTime, YQuoteItem, YahooConnector};

pub async fn search(
    yahoo_connector: &YahooConnector,
    symbol: &str,
) -> anyhow::Result<Vec<YQuoteItem>> {
    let response = yahoo_connector.search_ticker(symbol).await;

    Ok(response?.quotes)
}

pub async fn get_history(
    yahoo_connector: Arc<YahooConnector>,
    symbol: String,
    selected_symbol_history: Arc<Mutex<Option<Vec<f64>>>>,
) {
    let start: OffsetDateTime = OffsetDateTime::now_utc().add(Duration::days(-7 * 7));

    let end: OffsetDateTime = OffsetDateTime::now_utc();

    match yahoo_connector.get_quote_history(&symbol, start, end).await {
        Ok(history) => {
            let quotes = match history.quotes() {
                Ok(quotes) => quotes,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let close: Vec<f64> = quotes.iter().map(|quote| quote.close).collect();

            *selected_symbol_history.lock().unwrap() =
                Some(close.iter().copied().collect());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
