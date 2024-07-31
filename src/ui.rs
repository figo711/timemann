use std::time::Duration;


use ratatui::prelude::*;
use tui_big_text::{BigText, PixelSize};

use crate::{colors::BG_MAIN_COLOR, states::AppState};

pub fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::vertical(vec![
        Constraint::Length(1), // top bar
        Constraint::Min(0),    // splits
        Constraint::Length(1), // help
    ])
    .split(area);
    let top_layout = Layout::horizontal(vec![
        Constraint::Length(15), // title
        Constraint::Ratio(1, 2), // tabs
        Constraint::Length(15), // fps counter
    ])
    .flex(layout::Flex::SpaceBetween)
    .split(layout[0]);

    // return a new vec with the top_layout rects and then rest of layout
    top_layout[..]
        .iter()
        .chain(layout[1..].iter())
        .copied()
        .collect()
}

pub fn format_duration(duration: Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        duration.as_secs() / 3600,
        (duration.as_secs() / 60) % 60,
        duration.as_secs() % 60,
        duration.as_millis() % 1000
    )
}

pub fn timer_style(state: AppState) -> Style {
    if state.is_running() {
        Style::new().green().bg(BG_MAIN_COLOR)
    } else {
        Style::new().red()
    }
}

pub fn setup_big_timer_text(duration: String, state: AppState, width: u16) -> BigText<'static> {
    let pixel_size = match width {
        150.. => PixelSize::HalfHeight,
        _ => PixelSize::Sextant
    };

    let lines = vec![duration.into()];
        BigText::builder()
            // HalfHeight - h1 style
            // Quadrant - h2 style
            // Sextant - h3 style
            .pixel_size(pixel_size)
            .lines(lines)
            .style(timer_style(state))
            .alignment(Alignment::Center)
            .build()
            .unwrap()
}