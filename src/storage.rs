use std::{
    future::Future,
    pin::Pin,
    {sync::Arc, sync::Mutex},
};

use egui::ahash::HashMap;

use yahoo_finance_api::Quote;

use crate::{app::RUNTIME, requests::get_history};

#[derive(Default, Clone)]
pub struct Storage(Arc<tokio::sync::Mutex<StorageInner>>);

impl Storage {
    pub fn access(
        &self,
        function: impl FnOnce(Self) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static,
    ) {
        let storage = self.clone();

        RUNTIME.spawn(async move {
            let _ = Box::pin(function(storage).await);
        });
    }

    pub fn update_quotes_checked(&self, symbol: &str) {
        let symbol = symbol.to_string();

        self.access(move |storage| {
            Box::pin(async move {
                let mut storage = storage.inner().await;

                storage.quotes.update_checked(&symbol).await;
            })
        });
    }

    pub async fn inner(&self) -> tokio::sync::MutexGuard<StorageInner> {
        self.0.lock().await
    }

    #[allow(dead_code)]
    pub fn inner_blocking(&self) -> tokio::sync::MutexGuard<StorageInner> {
        self.0.blocking_lock()
    }

    pub fn get_quotes(&self, symbol: &str) -> Option<Vec<Quote>> {
        let storage = self.clone();
        let symbol = symbol.to_string();

        let history = Arc::new(Mutex::new(None));

        let cloned_history = history.clone();

        RUNTIME.block_on(async {
            let storage = storage.inner().await;

            let history = storage.quotes.history.get(&symbol).cloned();

            *cloned_history.lock().unwrap() = history;
        });

        let mut history = history.lock().unwrap();

        match history.take() {
            Some(history) => Some(history),
            None => None,
        }
    }
}

#[derive(Default)]
pub struct StorageInner {
    pub quotes: QuotesStorage,
}

#[derive(Default, Clone, Debug)]
pub struct QuotesStorage {
    history: HashMap<String, Vec<yahoo_finance_api::Quote>>,
    last_update: HashMap<String, std::time::SystemTime>,
}

const UPDATE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 24);

impl QuotesStorage {
    pub async fn update_checked(&mut self, symbol: &str) {
        let now = std::time::SystemTime::now();

        if let Some(last_update) = self.last_update.get(symbol) {
            if let Ok(elapsed) = now.duration_since(*last_update) {
                if elapsed < UPDATE_INTERVAL {
                    log::debug!("Skipping update for {}", symbol);
                    return;
                }
            }
        }

        let last_update_for_symbol = self
            .last_update
            .get(symbol)
            .cloned()
            .unwrap_or(std::time::UNIX_EPOCH);

        self.last_update.insert(symbol.to_string(), now);

        self.update(symbol, last_update_for_symbol).await;
    }

    pub async fn update(&mut self, symbol: &str, last_update: std::time::SystemTime) {
        // TODO: make this nicer / extract
        let end_datetime = yahoo_finance_api::time::OffsetDateTime::now_utc();

        let duration_since_last_update =
            match std::time::SystemTime::now().duration_since(last_update) {
                Ok(duration) => duration,
                Err(_) => {
                    log::error!("Error getting duration since last update");
                    return;
                }
            };

        let start_datetime =
            end_datetime.saturating_sub(yahoo_finance_api::time::Duration::seconds_f64(
                duration_since_last_update.as_secs_f64(),
            ));

        let history = Arc::new(Mutex::new(None));

        get_history(
            symbol.to_string(),
            history.clone(),
            start_datetime,
            end_datetime,
        )
        .await;

        let history = history.lock().unwrap();

        if let Some(history) = history.as_ref() {
            self.history.insert(symbol.to_string(), history.clone());
        }
    }
}
