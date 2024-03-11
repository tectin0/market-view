use egui_plot::{BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, Plot};
use yahoo_finance_api::Quote;

use super::ViewWindow;

pub struct PlotWindow {
    symbol: String,
    quotes: Vec<Quote>,
    id: String,
}

impl PlotWindow {
    pub fn new(symbol: String, quotes: Vec<Quote>) -> Self {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        PlotWindow { symbol, quotes, id }
    }
}

impl ViewWindow for PlotWindow {
    fn view(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Window::new(format!("{}", &self.symbol))
            .id(self.id.clone().into())
            .title_bar(true)
            .show(ui.ctx(), |ui| {
                let plot = Plot::new(format!("{}", &self.symbol))
                    .x_axis_formatter(|gridmark: egui_plot::GridMark, _, _| {
                        let date =
                            chrono::DateTime::from_timestamp(gridmark.value as i64, 0).unwrap();

                        date.format("%Y-%m-%d").to_string()
                    })
                    .label_formatter(|_, plot_point| {
                        format!(
                            "Date: {}\nValue: {}",
                            chrono::DateTime::from_timestamp(plot_point.x as i64, 0)
                                .unwrap()
                                .format("%Y-%m-%d")
                                .to_string(),
                            plot_point.y
                        )
                    })
                    .coordinates_formatter(
                        Corner::LeftTop,
                        CoordinatesFormatter::new(|plot_point, _| {
                            format!(
                                "Date: {}\nValue: {}",
                                chrono::DateTime::from_timestamp(plot_point.x as i64, 0)
                                    .unwrap()
                                    .format("%Y-%m-%d")
                                    .to_string(),
                                plot_point.y
                            )
                        }),
                    );

                plot.show(ui, |ui| {
                    let boxplot = BoxPlot::new(
                        self.quotes
                            .iter()
                            .map(|quote| {
                                let lower_whisker = quote.low;
                                let lower_quartile = quote.open.min(quote.close);
                                let median = (quote.open + quote.close) / 2.0;
                                let upper_quartile = quote.open.max(quote.close);
                                let upper_whisker = quote.high;

                                let color = if quote.open < quote.close {
                                    egui::Color32::from_rgb(0, 255, 0)
                                } else {
                                    egui::Color32::from_rgb(255, 0, 0)
                                };

                                BoxElem::new(
                                    quote.timestamp as f64,
                                    BoxSpread::new(
                                        lower_whisker,
                                        lower_quartile,
                                        median,
                                        upper_quartile,
                                        upper_whisker,
                                    ),
                                )
                                .whisker_width(0.0)
                                .box_width(40000.0)
                                .fill(color)
                                .stroke(egui::Stroke::new(1.0, color))
                            })
                            .collect::<Vec<_>>(),
                    )
                    .element_formatter(Box::new(|elem, _| {
                        let is_red = elem.fill == egui::Color32::from_rgb(255, 0, 0);

                        let (open, close) = if is_red {
                            (elem.spread.quartile1, elem.spread.quartile3)
                        } else {
                            (elem.spread.quartile3, elem.spread.quartile1)
                        };

                        let x = elem.argument as i64;

                        let x = chrono::DateTime::from_timestamp(x, 0)
                            .unwrap()
                            .format("%Y-%m-%d")
                            .to_string();

                        let low = elem.spread.lower_whisker;
                        let high = elem.spread.upper_whisker;

                        format!(
                            "Date: {}\n\nOpen: {:.2}\nClose: {:.2}\nHigh: {:.2}\nLow: {:.2}",
                            x, open, close, high, low
                        )
                    }));

                    ui.box_plot(boxplot);
                })
            });
    }
}
