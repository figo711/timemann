use std::ops::Add;
use std::time::Duration;

use anyhow::{Ok, Result};
use notify_rust::Notification;
use tui_big_text::BigText;
use ratatui::prelude::*;

use crate::tabs::AppTab;
use crate::{msg::Message, states::AppState, timer::Timer};
use crate::ui::*;


#[derive(Debug, Clone, PartialEq)]
pub struct CountdownTab {
    state: AppState,
    timer: Timer,
    init_duration: Duration,
    index_counter: u8,
    can_start: bool
}

impl CountdownTab {

    fn clear(&mut self) {
        if self.state.is_stopped() || self.state.is_setup() {
            self.index_counter = 0;
            self.init_duration = Duration::from_secs(0);
            self.timer.reset();

            self.update_can_start_value();
        }
    }

    fn toggle_start_pause(&mut self) {
        if !self.can_start {
            return;
        }

        if !self.state.is_running() {
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

    fn edit(&mut self) {
        self.stop();
        self.clear();
        self.state = AppState::Setup;
        self.can_start = false;
    }

    fn set_number(&mut self, num: u8) {
        if !self.state.is_setup() {
            return;
        }

        let high_num: u64 = num.into();

        let dur = match self.index_counter {
            0 => Duration::from_secs(high_num),
            1 => Duration::from_secs(high_num.wrapping_mul(10)),
            2 => Duration::from_secs(high_num.wrapping_mul(60)),
            3 => Duration::from_secs(high_num.wrapping_mul(600)),
            4 => Duration::from_secs(high_num.wrapping_mul(3600)),
            5 => Duration::from_secs(high_num.wrapping_mul(36000)),
            _ => Duration::from_secs(0)
        };

        self.init_duration = self.init_duration.add(dur);
        self.index_counter += 1;
        if self.index_counter > 5 {
            self.index_counter = 0;
        }

        self.update_can_start_value()
    }

    fn update_can_start_value(&mut self) {
        self.can_start = !self.init_duration.is_zero();
    }

    fn check_delta(&mut self) {
        if self.delta_option().is_none() {
            let _ = Notification::new()
                .summary("TimeMann Alert")
                .body("Countdown finished!")
                .show();

            self.stop();
            self.clear();
        }
    }

    fn delta_option(&self) -> Option<Duration> {
        self.init_duration.checked_sub(self.timer.elapsed())
    }

    fn delta(&mut self) -> Duration {
        let diff = match self.delta_option() {
            Some(d) => d,
            None => Duration::from_secs(0)
        };

        diff
    }

    fn timer_paragraph(&mut self, area: Rect) -> BigText<'_> {
        self.check_delta();

        setup_big_timer_text(
            format_duration(self.delta()), 
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

impl AppTab for CountdownTab {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::ToggleStartPause => self.toggle_start_pause(),
            Message::Clear => self.clear(),
            Message::SetNumber(num) => self.set_number(num),
            Message::Edit => self.edit(),
            _ => (),
        }
    }
    
    fn render_paragraph(&mut self, frame: &mut Frame, area: &Rect) -> Result<()> {
        let area = layout(*area);
        frame.render_widget(self.timer_paragraph(area[0]), area[0]);
        Ok(())
    }

    fn help_text(&mut self) -> Vec<Span> {
        let enter_part = if self.can_start {
            ( 
                " Enter ".on_dark_gray().black().dim(), 
                if !self.state.is_running() {
                    " Start ".dim()
                } else {
                    " Pause ".dim()
                }
            )
        } else {
            ( "".on_dark_gray().black().dim(), "".dim() )
        };

        let edit_part = if !self.state.is_setup() {
            ( " E ".on_dark_gray().black().dim(), " Edit Mode ".dim() )       
        } else {
            ( "".on_dark_gray().black().dim(), "".dim() )
        };

        let number_part = if self.state.is_setup() {
            ( " 0-9 ".on_dark_gray().black().dim(), " Set Time ".dim() )           
        } else {
            ( "".on_dark_gray().black().dim(), "".dim() )
        };

        let clear_part = if self.state.is_setup() && !self.init_duration.is_zero() {
            ( " C ".on_dark_gray().black().dim(), " Clear ".dim() )
        } else {
            ( "".on_dark_gray().black().dim(), "".dim() )
        };

        vec![
            enter_part.0,
            enter_part.1,
            edit_part.0,
            edit_part.1,
            number_part.0,
            number_part.1,
            clear_part.0,
            clear_part.1
        ]
    }
}

impl Default for CountdownTab {
    fn default() -> Self {
        Self {
            state: AppState::Setup,
            timer: Timer::new(),
            init_duration: Duration::from_secs(0),
            index_counter: 0,
            can_start: false
        }
    }
}