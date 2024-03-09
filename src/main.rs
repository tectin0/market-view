use eframe::NativeOptions;
use egui::ViewportBuilder;

mod app;
mod plotter;
mod requests;
mod windows;

#[tokio::main]
async fn main() {
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
