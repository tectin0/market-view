use std::sync::Arc;
use std::sync::Mutex;

use egui::Vec2;
use yahoo_finance_api::YahooConnector;

use crate::windows::PlotWindow;
use crate::windows::SearchWindow;
use crate::windows::ViewWindow;

pub struct App {
    yahoo_connector: Arc<YahooConnector>,
    search_window: SearchWindow,
    plot_windows: Arc<Mutex<Vec<PlotWindow>>>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        App::default()
    }
}

impl Default for App {
    fn default() -> Self {
        let yahoo_connector = Arc::new(YahooConnector::new());

        let plot_windows = Arc::new(Mutex::new(Vec::new()));

        let search_window = SearchWindow::new(plot_windows.clone(), yahoo_connector.clone());

        App {
            yahoo_connector,
            search_window,
            plot_windows,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Main Top Panel").show(ctx, |ui| {
            let available_width = ui.available_width();
            let available_height = ui.available_height();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(available_width, available_height),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.button("X")
                            .on_hover_text("Close the application")
                            .clicked()
                            .then(|| {
                                std::process::exit(0);
                            });
                    },
                );

                ui.allocate_ui_with_layout(
                    Vec2::new(available_width * 0.99, available_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label("Market View");
                    },
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.search_window.view(ui);

            for plot_window in self.plot_windows.lock().unwrap().iter_mut() {
                plot_window.view(ui);
            }
        });
    }
}
