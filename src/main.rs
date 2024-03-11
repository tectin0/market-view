#![feature(lazy_cell)]

use eframe::NativeOptions;
use egui::ViewportBuilder;

mod app;
mod requests;
mod storage;
mod windows;

fn main() {
    let _ = simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_level("market_view", log::LevelFilter::Debug)
        .init();

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_maximized(true)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Market View",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
    .expect("Failed to run native");
}
