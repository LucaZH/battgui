pub mod state;

use iced::font;

#[derive(Debug)]
pub enum Message {
    Tick,
    FontLoaded(Result<(), font::Error>),
}

pub const SAMPLE_EVERY: std::time::Duration = std::time::Duration::from_millis(1000);
