use anyhow::Context;
use egui::Sense;
use egui_plot::{
    BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, Line, Plot, PlotPoints,
};
use yahoo_finance_api::Quote;

use super::ViewWindow;

struct LineInfo {
    start: [f64; 2],
    end: [f64; 2],
    is_fixed: bool,
}

pub struct PlotWindow {
    symbol: String,
    quotes: Vec<Quote>,
    id: String,
    request_close: bool,
    line_info: Option<LineInfo>,
    lines: Vec<LineInfo>,
}

impl PlotWindow {
    pub fn new(symbol: String, quotes: Vec<Quote>) -> Self {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        PlotWindow {
            symbol,
            quotes,
            id,
            request_close: false,
            line_info: None,
            lines: vec![],
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn is_request_close(&self) -> bool {
        self.request_close
    }

    fn new_plot_window(&mut self) -> Plot {
        let plot = Plot::new(format!("{}", &self.symbol))
            .x_axis_formatter(|gridmark: egui_plot::GridMark, _, _| {
                let date = chrono::DateTime::from_timestamp(gridmark.value as i64, 0).unwrap();

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
        plot
    }

    fn plot_show(
        &mut self,
        plot: Plot,
        ui: &mut egui::Ui,
        transform: &mut Option<egui_plot::PlotTransform>,
    ) -> egui_plot::PlotResponse<()> {
        let plot_response = plot.show(ui, |ui| {
            *transform = Some(*ui.transform());

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

            for line in self.lines.iter() {
                ui.line(Line::new(PlotPoints::new(
                    [line.start, line.end]
                        .iter()
                        .map(|pos| [pos[0], pos[1]])
                        .collect::<Vec<_>>(),
                )));
            }

            if let Some(line_info) = &self.line_info {
                ui.line(Line::new(PlotPoints::new(
                    [line_info.start, line_info.end]
                        .iter()
                        .map(|pos| [pos[0], pos[1]])
                        .collect::<Vec<_>>(),
                )))
            }
        });
        plot_response
    }
}

impl ViewWindow for PlotWindow {
    fn view(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Window::new(format!("{}", &self.symbol))
            .id(self.id.clone().into())
            .title_bar(true)
            .show(ui.ctx(), |ui| {
                let plot = self.new_plot_window();

                let mut transform = None;

                let plot_response = self.plot_show(plot, ui, &mut transform);

                let plot_rect = plot_response.response.rect;

                let hover_pos = plot_response.response.hover_pos();

                if plot_response.response.clicked() {
                    match &mut self.line_info {
                        None => {
                            log::debug!("Drag started");
                            if let Some(screen_pos) = hover_pos {
                                log::debug!("Clicked at {:?}", screen_pos);

                                let plot_pos = match transform {
                                    Some(transform) => transform.value_from_position(screen_pos),
                                    None => return,
                                };

                                self.line_info = Some(LineInfo {
                                    start: [plot_pos.x, plot_pos.y],
                                    end: [plot_pos.x, plot_pos.y],
                                    is_fixed: false,
                                });
                            }
                        }
                        Some(line_info) => {
                            log::debug!("Drag ended");
                            if let Some(screen_pos) = hover_pos {
                                log::debug!("Clicked at {:?}", screen_pos);

                                let plot_pos = match transform {
                                    Some(transform) => transform.value_from_position(screen_pos),
                                    None => return,
                                };

                                line_info.end = [plot_pos.x, plot_pos.y];

                                line_info.is_fixed = true;
                            }
                        }
                    }

                    if self.line_info.as_ref().unwrap().is_fixed {
                        self.lines.push(self.line_info.take().unwrap());
                    }
                }

                if let Some(line_info) = &mut self.line_info {
                    if !line_info.is_fixed {
                        if let Some(screen_pos) = hover_pos {
                            let plot_pos = match transform {
                                Some(transform) => transform.value_from_position(screen_pos),
                                None => return,
                            };

                            line_info.end = [plot_pos.x, plot_pos.y];
                        }
                    }
                }
            })
            .unwrap()
            .response
            .double_clicked()
            .then(|| {
                self.request_close = true;
            });
    }
}
