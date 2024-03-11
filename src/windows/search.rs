use std::sync::{Arc, Mutex};

use yahoo_finance_api::{Quote, YQuoteItem};

use crate::{
    app::{RUNTIME, STORAGE},
    requests,
};

use super::{PlotWindow, ViewWindow};

pub struct SearchWindow {
    search_string: String,
    search_results: Arc<Mutex<Vec<YQuoteItem>>>,
    selected_symbol: Option<String>,
    selected_symbol_history: Arc<Mutex<Option<Vec<Quote>>>>,
    plot_windows: Arc<Mutex<Vec<PlotWindow>>>,
}

impl SearchWindow {
    pub fn new(plot_windows: Arc<Mutex<Vec<PlotWindow>>>) -> Self {
        SearchWindow {
            search_string: String::new(),
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
                            
                            let search_results = self.search_results.clone();

                            RUNTIME.spawn(async move {
                                let results = requests::search( &search_string)
                                    .await
                                    .unwrap_or_default();

                                *search_results.lock().unwrap() = results;
                            });
                        }

                        let search_results = self.search_results.lock().unwrap();

                        for result in search_results.iter() {
                            let response = ui.button(result.long_name.clone());

                            if response.clicked() {
                                self.selected_symbol = Some(result.symbol.clone());
                                
                                let symbol = result.symbol.clone();

                                STORAGE.update_quotes_checked(&symbol);
                            }

                            response.on_hover_text(format!(
                                "Symbol: {}\nShort Name: {}\nExchange: {}\nQuote Type: {}\nScore: {}",
                                result.symbol,
                                result.short_name,
                                result.exchange,
                                result.quote_type,
                                result.score
                            ));
                        }
                    });

                    if let Some(selected_symbol) = &self.selected_symbol {
                        let history = STORAGE.get_quotes(selected_symbol);

                        if let Some(history) = history {
                            *self.selected_symbol_history.lock().unwrap() = Some(history);
                        }
                    }

                    if let Some(history) = self.selected_symbol_history.lock().unwrap().take() {
                        let plot_window =
                            PlotWindow::new(self.selected_symbol.clone().unwrap(), history);

                        self.plot_windows.lock().unwrap().push(plot_window);

                        self.selected_symbol = None;
                    }
                });
            });
    }
}
