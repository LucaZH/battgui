mod app;
mod battery;
mod chart;
mod ui;

use app::state::State;
use iced::font;
use std::time::Duration;

fn main() {
    iced::application("BattGUI", State::update, State::view)
        .antialiasing(true)
        .default_font(font::Font::with_name("Noto Sans"))
        .subscription(|_| {
            const FPS: u64 = 50;
            iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| app::Message::Tick)
        })
        .run_with(State::new)
        .unwrap();
}
