use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use egui::Vec2;

use yahoo_finance_api::YahooConnector;

use crate::storage::Storage;
use crate::windows::PlotWindow;
use crate::windows::SearchWindow;
use crate::windows::ViewWindow;

pub static CONNECTOR: LazyLock<YahooConnector> = LazyLock::new(|| YahooConnector::new());

pub static STORAGE: LazyLock<Storage> = LazyLock::new(|| Storage::default());

pub static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
});

pub struct App {
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
        let plot_windows = Arc::new(Mutex::new(Vec::new()));

        let search_window = SearchWindow::new(plot_windows.clone());

        STORAGE.update_quotes_checked("NIO");

        App {
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

            let mut window_id_to_remove = None;

            for plot_window in self.plot_windows.lock().unwrap().iter_mut() {
                if !plot_window.is_request_close() {
                    plot_window.view(ui);
                } else {
                    window_id_to_remove = Some(plot_window.id().to_string());
                }
            }

            if let Some(window_id_to_remove) = window_id_to_remove {
                self.plot_windows
                    .lock()
                    .unwrap()
                    .retain(|plot_window| plot_window.id() != window_id_to_remove);
            }
        });
    }
}
