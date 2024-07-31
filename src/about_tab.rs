use anyhow::{Ok, Result};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::tabs::AppTab;
use crate::msg::Message;
use crate::colors::BG_MAIN_COLOR;


pub static LOGO: &str = include_str!("../assets/logo.txt");

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AboutTab {
}

impl AboutTab {

    fn title_paragraph(&mut self) -> Paragraph<'_> {
        Paragraph::new(LOGO)
            .bg(BG_MAIN_COLOR)
            .centered()
    }

    fn link_paragraph(&mut self) -> Paragraph<'_> {
        Paragraph::new("Source code: https://github.com/figo711/timemann")
            .dim()
            .centered()
    }

}

pub fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::vertical(vec![
        Constraint::Length(3), // INGORE
        Constraint::Length(13), // timer
        Constraint::Length(1), // SPACE
        Constraint::Length(1), // Text Link
        Constraint::Length(3), // INGORE
    ])
    .split(area);

    layout[1..]
        .iter()
        .copied()
        .collect()
}

impl AppTab for AboutTab {
    fn handle_message(&mut self, message: Message) {
        match message {
            _ => (),
        }
    }
    
    fn render_paragraph(&mut self, frame: &mut Frame, area: &Rect) -> Result<()> {
        let area = layout(*area);
        frame.render_widget(self.title_paragraph(), area[0]);
        frame.render_widget(self.link_paragraph(), area[2]);
        Ok(())
    }

    fn help_text(&mut self) -> Vec<Span> {
        vec![]
    }
}
