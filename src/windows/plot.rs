use egui_plot::{Line, Plot};

use super::ViewWindow;

pub struct PlotWindow {
    symbol: String,
    x: Vec<f64>,
    y: Vec<f64>,
    id: String,
}

impl PlotWindow {
    pub fn new(symbol: String, x: Vec<f64>, y: Vec<f64>) -> Self {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        PlotWindow { symbol, x, y, id }
    }
}

impl ViewWindow for PlotWindow {
    fn view(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Window::new(format!("{}", &self.symbol))
            .id(self.id.clone().into())
            .title_bar(true)
            .show(ui.ctx(), |ui| {
                let plot = Plot::new(format!("{}", &self.symbol));

                plot.show(ui, |ui| {
                    let line = Line::new(
                        itertools::izip!(self.x.iter(), self.y.iter())
                            .map(|(x, y)| [*x, *y])
                            .collect::<Vec<[f64; 2]>>(),
                    );

                    ui.line(line);
                })
            });
    }
}
