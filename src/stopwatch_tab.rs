use anyhow::{Ok, Result};
use tui_big_text::BigText;
use ratatui::prelude::*;

use crate::tabs::AppTab;
use crate::ui::{format_duration, setup_big_timer_text};
use crate::{msg::Message, states::AppState, timer::Timer};


#[derive(Debug, Default, Clone, PartialEq)]
pub struct StopwatchTab {
    state: AppState,
    timer: Timer,
}

impl StopwatchTab {

    fn clear(&mut self) {
        if self.state.is_stopped() {
            self.timer.reset();
        }
    }

    fn toggle_start_pause(&mut self) {
        if self.state.is_stopped() {
            self.start();
        } else {
            self.stop();
        }
    }

    fn start(&mut self) {
        self.timer.start();
        self.state = AppState::Running;
    }

    fn stop(&mut self) {
        self.timer.pause();
        self.state = AppState::Stopped;
    }

    fn timer_paragraph(&mut self, area: Rect) -> BigText<'_> {
        setup_big_timer_text(
            format_duration(self.timer.elapsed()), 
            self.state, 
            area.width
        )
    }

}

pub fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::vertical(vec![
        Constraint::Percentage(100), // INGORE
        Constraint::Min(8), // timer
        Constraint::Percentage(100), // INGORE
    ])
    .split(area);

    layout[1..]
        .iter()
        .copied()
        .collect()
}

impl AppTab for StopwatchTab {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::ToggleStartPause => self.toggle_start_pause(),
            Message::Clear => self.clear(),
            // Message::Stop => self.stop(),
            _ => (),
        }
    }
    
    fn render_paragraph(&mut self, frame: &mut Frame, area: &Rect) -> Result<()> {
        let area = layout(*area);
        frame.render_widget(self.timer_paragraph(area[0]), area[0]);
        Ok(())
    }

    fn help_text(&mut self) -> Vec<Span> {
        let enter_action = if self.state.is_stopped() {
            " Start "
        } else {
            " Pause "
        };

        let clear_part = if self.state.is_stopped() && !self.timer.elapsed().is_zero() {
            ( " C ".on_dark_gray().black().dim(), " Clear ".dim() )
        } else {
            ( "".on_dark_gray().black().dim(), "".dim() )
        };

        vec![
            " Enter ".on_dark_gray().black().dim(),
            enter_action.dim(),
            clear_part.0,
            clear_part.1
        ]
    }
}