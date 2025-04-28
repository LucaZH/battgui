use crate::app::Message;
use crate::app::SAMPLE_EVERY;
use crate::battery::info::BatteryInfo;
use crate::chart::energy_rate::EnergyRateChart;
use crate::ui::components::*;
use battery::{Manager, State as BatteryState};
use chrono::Utc;
use iced::{
    Alignment, Color, Element, Font, Length, Task, font,
    widget::{Column, Container, Row, Text},
};
use plotters_iced::ChartWidget;
use std::time::Instant;

pub const TITLE_FONT_SIZE: u16 = 22;
pub const SUB_TITLE_FONT_SIZE: u16 = 18;

pub const FONT_BOLD: Font = Font {
    family: font::Family::Name("Noto Sans"),
    weight: font::Weight::Bold,
    ..Font::DEFAULT
};

pub struct State {
    battery_manager: Manager,
    battery_info: Vec<BatteryInfo>,
    last_sample_time: Instant,
    energy_rate_chart: EnergyRateChart,
    batteries_initialized: bool,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let manager = Manager::new().expect("Failed to create battery manager");
        (
            Self {
                battery_manager: manager,
                battery_info: Vec::new(),
                last_sample_time: Instant::now(),
                energy_rate_chart: EnergyRateChart::default(),
                batteries_initialized: false,
            },
            Task::batch([
                font::load(include_bytes!("../fonts/notosans-regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../fonts/notosans-bold.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.update_battery_info();
            }
            _ => {}
        }
    }

    fn update_battery_info(&mut self) {
        if !self.should_update() {
            return;
        }

        let now = Utc::now();
        self.last_sample_time = Instant::now();

        let batteries = self
            .battery_manager
            .batteries()
            .expect("Failed to get batteries");
        let mut updated_info = Vec::new();

        for battery in batteries {
            let battery = battery.expect("Failed to access battery");

            let battery_info = BatteryInfo::from_battery(&battery);
            updated_info.push(battery_info);
        }

        if !self.batteries_initialized && !updated_info.is_empty() {
            self.batteries_initialized = true;
            for info in &updated_info {
                self.energy_rate_chart
                    .push_data(now.into(), info.energy_rate);
            }
        } else if !updated_info.is_empty() {
            let avg_energy_rate =
                updated_info.iter().map(|b| b.energy_rate).sum::<f32>() / updated_info.len() as f32;
            self.energy_rate_chart
                .push_data(now.into(), avg_energy_rate);
        }

        self.battery_info = updated_info;
    }

    fn should_update(&self) -> bool {
        self.last_sample_time.elapsed() > SAMPLE_EVERY
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut content = Column::new()
            .spacing(20)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .push(Text::new("BattGUI").size(TITLE_FONT_SIZE).font(FONT_BOLD));

        if self.battery_info.is_empty() {
            content = content.push(
                Text::new("No batteries detected or still initializing...")
                    .size(SUB_TITLE_FONT_SIZE),
            );
        } else {
            content = content.push(
                Column::new()
                    .spacing(10)
                    .width(Length::Fill)
                    .push(
                        Text::new("Energy Rate (Watts)")
                            .size(SUB_TITLE_FONT_SIZE)
                            .font(FONT_BOLD),
                    )
                    .push(ChartWidget::new(&self.energy_rate_chart).height(Length::FillPortion(2))),
            );

            for (idx, info) in self.battery_info.iter().enumerate() {
                let battery_column = Column::new()
                    .spacing(15)
                    .width(Length::Fill)
                    .push(
                        Text::new(format!("Battery {} - {}", idx + 1, info.name))
                            .size(SUB_TITLE_FONT_SIZE)
                            .font(FONT_BOLD),
                    )
                    .push(self.battery_info_view(info));

                content = content.push(battery_column);
            }
        }

        Container::new(content)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn battery_info_view(&self, info: &BatteryInfo) -> Element<Message> {
        let time_remaining = match info.state {
            BatteryState::Discharging => info
                .time_to_empty
                .map_or("Unknown".to_string(), format_duration),
            BatteryState::Charging => info
                .time_to_full
                .map_or("Unknown".to_string(), format_duration),
            _ => "N/A".to_string(),
        };

        let status_text = match info.state {
            BatteryState::Charging => format!("Charging - {} until full", time_remaining),
            BatteryState::Discharging => format!("Discharging - {} remaining", time_remaining),
            BatteryState::Full => "Fully Charged".to_string(),
            _ => format!("{:?}", info.state),
        };

        let status_color = match info.state {
            BatteryState::Charging => Color::from_rgb(0.2, 0.8, 0.2),
            BatteryState::Discharging => {
                if info.percentage < 20.0 {
                    Color::from_rgb(0.8, 0.2, 0.2)
                } else {
                    Color::from_rgb(0.2, 0.2, 0.8)
                }
            }
            BatteryState::Full => Color::from_rgb(0.0, 0.7, 0.0),
            _ => Color::BLACK,
        };

        let left = Column::new()
            .spacing(5)
            .push(info_row(
                "Health",
                &format!(
                    "{:.1}%",
                    (info.energy_full * 100.00) / info.energy_full_design
                ),
            ))
            .push(info_row("Charge", &format!("{:.1}%", info.percentage)))
            .push(info_row("Voltage", &format!("{:.2} V", info.voltage)))
            .push(info_row(
                "Energy Rate",
                &format!("{:.2} W", info.energy_rate),
            ))
            .push(info_row(
                "Energy",
                &format!("{:.2} Wh / {:.2} Wh", info.energy, info.energy_full),
            ))
            .push(info_row(
                "Energy Full design",
                &format!("{:.2} W", info.energy_full_design),
            ));

        let middle = Column::new()
            .spacing(5)
            .push(info_row("Technology", &info.technology))
            .push(info_row("Model", &info.model))
            .push(info_row(
                "Cycle Count",
                &info
                    .cycle_count
                    .map_or("Unknown".to_string(), |c| c.to_string()),
            ));

        let right = Column::new().spacing(5).push(info_row(
            "Temperature",
            &info
                .temperature
                .map_or("Unknown".to_string(), |t| format!("{:.1} °C", t)),
        ));

        Column::new()
            .spacing(10)
            .width(Length::Fill)
            .push(Text::new(status_text).color(status_color).size(18))
            .push(
                Row::new()
                    .spacing(20)
                    .width(Length::Fill)
                    .push(left.width(Length::FillPortion(1)))
                    .push(middle.width(Length::FillPortion(1)))
                    .push(right.width(Length::FillPortion(1))),
            )
            .into()
    }
}
