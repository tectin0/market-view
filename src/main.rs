mod app;
mod plotter;
mod requests;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Market View",
        native_options,
        Box::new(|cc| Box::new(app::App::new(&cc))),
    )
    .expect("Failed to run native");
}
