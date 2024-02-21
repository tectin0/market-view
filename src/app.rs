use std::sync::Arc;
use std::sync::Mutex;

use yahoo_finance_api::{YQuoteItem, YahooConnector};

use crate::requests;

pub struct App {
    yahoo_connector: Arc<YahooConnector>,
    search_string: String,
    search_results: Arc<Mutex<Vec<YQuoteItem>>>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        App::default()
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            yahoo_connector: Arc::new(YahooConnector::new()),
            search_string: String::new(),
            search_results: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, _ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.heading("Yahoo Finance API");
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut self.search_string);

                if response.lost_focus() {
                    let search_string = self.search_string.clone();
                    let yahoo_connector = self.yahoo_connector.clone();
                    let search_results = self.search_results.clone();

                    tokio::task::spawn(async move {
                        let results = requests::search(&yahoo_connector, &search_string)
                            .await
                            .unwrap_or_default();

                        *search_results.lock().unwrap() = results;
                    });
                }
            });

            let search_results = self.search_results.lock().unwrap();

            for result in search_results.iter() {
                ui.label(format!("{}", result.symbol));
            }
        });
    }
}
