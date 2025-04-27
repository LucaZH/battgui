use crate::app::Message;
use crate::chart::PLOT_SECONDS;
use chrono::{DateTime, Utc};
use iced::{
    Size,
    widget::canvas::{Cache, Frame, Geometry},
};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, Renderer, plotters_backend::DrawingBackend};
use std::{collections::VecDeque, time::Duration};

pub struct EnergyRateChart {
    cache: Cache,
    data_points: VecDeque<(DateTime<Utc>, f32)>,
    limit: Duration,
}

impl Default for EnergyRateChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            data_points: VecDeque::new(),
            limit: Duration::from_secs(PLOT_SECONDS as u64),
        }
    }
}

impl EnergyRateChart {
    pub fn push_data(&mut self, time: DateTime<Utc>, value: f32) {
        let cur_ms = time.timestamp_millis();
        self.data_points.push_front((time, value));

        while let Some((time, _)) = self.data_points.back() {
            let diff = Duration::from_millis((cur_ms - time.timestamp_millis()) as u64);
            if diff > self.limit {
                self.data_points.pop_back();
            } else {
                break;
            }
        }

        self.cache.clear();
    }
}

impl Chart<Message> for EnergyRateChart {
    type State = ();

    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        if self.data_points.is_empty() {
            return;
        }

        let min_y = self
            .data_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::INFINITY, f32::min);
        let max_y = self
            .data_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::NEG_INFINITY, f32::max);
        let margin = 5.0;
        let y_min = (min_y - margin).floor().max(0.0);
        let y_max = (max_y + margin).ceil();

        let (start_time, end_time) = (
            self.data_points.back().unwrap().0,
            self.data_points.front().unwrap().0,
        );

        let mut root = chart
            .margin(10)
            .caption("Energy Rate Over Time", ("sans-serif", 20).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(start_time..end_time, y_min..y_max)
            .unwrap();

        root.configure_mesh()
            .x_labels(5)
            .x_label_formatter(&|dt| dt.format("%H:%M:%S").to_string())
            .x_desc("Time")
            .y_desc("Energy Rate (W)")
            .y_labels(5)
            .light_line_style(&WHITE.mix(0.3))
            .axis_style(&WHITE.mix(0.8))
            .label_style(("sans-serif", 14))
            .draw()
            .unwrap();

        root.draw_series(
            AreaSeries::new(
                self.data_points.iter().map(|(x, y)| (*x, *y)),
                y_min,
                RGBColor(0, 175, 255).mix(0.3),
            )
            .border_style(&BLUE.mix(0.8)),
        )
        .unwrap();
    }
}
