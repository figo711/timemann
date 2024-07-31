use std::time::Duration;

use anyhow::{bail, Result};
use crossterm::event::{self, KeyCode};
use tokio::select;
use futures::{FutureExt, StreamExt};

use crate::msg::Message;


/// Handles events from crossterm and emits `Message`s.
pub struct EventHandler {
    crossterm_events: event::EventStream,
    interval: tokio::time::Interval,
}

impl EventHandler {
    /// Creates a new event handler that emits a `Message::Tick` every `1.0 / max_fps` seconds.
    pub fn new(max_fps: f32) -> Self {
        let period = Duration::from_secs_f32(1.0 / max_fps);
        Self {
            crossterm_events: event::EventStream::new(),
            interval: tokio::time::interval(period),
        }
    }

    pub async fn next(&mut self) -> Result<Message> {
        select! {
            event = self.crossterm_events.next().fuse() => Self::handle_crossterm_event(event),
            _ = self.interval.tick().fuse() => Ok(Message::Tick),
        }
    }

    fn handle_crossterm_event(
        event: Option<core::result::Result<event::Event, std::io::Error>>,
    ) -> Result<Message> {
        match event {
            Some(Ok(event::Event::Key(key))) => Ok(match key.code {
                KeyCode::Enter => Message::ToggleStartPause,
                KeyCode::Tab => Message::ChangeTab,
                KeyCode::Char('e') => Message::Edit,
                KeyCode::Char(cey) => match cey {
                    '0'..='9' => Message::SetNumber(cey
                        .to_digit(10)
                        .unwrap()
                        .try_into()
                        .unwrap()),
                    'a' => Message::ChangeTab,
                    'q' => Message::Quit,
                    'c' => Message::Clear,
                    _ => Message::Tick,
                }
                _ => Message::Tick,
            }),
            Some(Err(err)) => bail!(err),
            None => bail!("event stream ended unexpectedly"),
            _ => Ok(Message::Tick),
        }
    }
}