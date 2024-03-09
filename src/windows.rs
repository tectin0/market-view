mod plot;
mod search;

pub use plot::PlotWindow;
pub use search::SearchWindow;

pub trait ViewWindow {
    fn view(&mut self, ui: &mut eframe::egui::Ui);
}
