use std::sync::{Arc, Mutex};

use yahoo_finance_api::Quote;
use yahoo_finance_api::{time::OffsetDateTime, YQuoteItem};

use crate::app::CONNECTOR;

pub async fn search(symbol: &str) -> anyhow::Result<Vec<YQuoteItem>> {
    let response = CONNECTOR.search_ticker(symbol).await;

    Ok(response?.quotes)
}

pub async fn get_history(
    symbol: String,
    selected_symbol_history: Arc<Mutex<Option<Vec<Quote>>>>,
    start: OffsetDateTime,
    end: OffsetDateTime,
) {
    match CONNECTOR.get_quote_history(&symbol, start, end).await {
        Ok(history) => {
            let quotes = match history.quotes() {
                Ok(quotes) => quotes,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            *selected_symbol_history.lock().unwrap() = Some(quotes);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
