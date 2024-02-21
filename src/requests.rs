use anyhow::Context;
use yahoo_finance_api::{YQuoteItem, YahooConnector};

pub async fn search(
    yahoo_connector: &YahooConnector,
    symbol: &str,
) -> anyhow::Result<Vec<YQuoteItem>> {
    let response = yahoo_connector.search_ticker(symbol).await;

    Ok(response?.quotes)
}
