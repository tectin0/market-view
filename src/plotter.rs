use egui_plot::{Line, Plot, PlotPoints};

pub fn plot(ui: &mut egui::Ui, data: &Vec<f64>) {
    let plot_points: PlotPoints = data
        .iter()
        .enumerate()
        .map(|(i, &v)| [i as f64, v])
        .collect();

    let plot = Plot::new("Stock Prices");

    plot.show(ui, |ui| {
        ui.line(
            Line::new(plot_points)
                .name("Stock Prices")
                .color(egui::Color32::from_rgb(0, 255, 0)),
        );
    });
}
