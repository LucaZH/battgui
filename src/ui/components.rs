use crate::app::Message;
use iced::{
    Element,
    widget::{Row, Text},
};
use std::time::Duration;

pub fn info_row(label: &str, value: &str) -> Element<'static, Message> {
    Row::new()
        .spacing(10)
        .push(Text::new(format!("{}:", label)).size(16))
        .push(Text::new(value.to_string()).size(16))
        .into()
}

pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
