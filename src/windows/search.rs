use std::sync::{Arc, Mutex};

use yahoo_finance_api::{YQuoteItem, YahooConnector};

use crate::requests;

use super::{PlotWindow, ViewWindow};

pub struct SearchWindow {
    search_string: String,
    yahoo_connector: Arc<YahooConnector>,
    search_results: Arc<Mutex<Vec<YQuoteItem>>>,
    selected_symbol: Option<String>,
    selected_symbol_history: Arc<Mutex<Option<Vec<f64>>>>,
    plot_windows: Arc<Mutex<Vec<PlotWindow>>>,
}

impl SearchWindow {
    pub fn new(
        plot_windows: Arc<Mutex<Vec<PlotWindow>>>,
        yahoo_connector: Arc<YahooConnector>,
    ) -> Self {
        SearchWindow {
            search_string: String::new(),
            yahoo_connector,
            search_results: Arc::new(Mutex::new(Vec::new())),
            selected_symbol: None,
            selected_symbol_history: Arc::new(Mutex::new(None)),
            plot_windows,
        }
    }
}

impl ViewWindow for SearchWindow {
    fn view(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Window::new("Search Window")
            .title_bar(false)
            .show(ui.ctx(), |ui| {
                ui.heading("Search");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
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

                        let search_results = self.search_results.lock().unwrap();

                        for result in search_results.iter() {
                            if ui.button(result.symbol.clone()).clicked() {
                                self.selected_symbol = Some(result.symbol.clone());

                                let yahoo_connector = self.yahoo_connector.clone();
                                let symbol = result.symbol.clone();
                                let selected_symbol_history = self.selected_symbol_history.clone();

                                tokio::task::spawn(async move {
                                    requests::get_history(
                                        yahoo_connector,
                                        symbol,
                                        selected_symbol_history,
                                    )
                                    .await;
                                });
                            }
                        }
                    });

                    if let Some(history) = self.selected_symbol_history.lock().unwrap().take() {
                        let plot_window = PlotWindow::new(
                            self.selected_symbol.clone().unwrap(),
                            (0..history.len()).map(|v| v as f64).collect(),
                            history,
                        );

                        self.plot_windows.lock().unwrap().push(plot_window);
                    }
                });
            });
    }
}
