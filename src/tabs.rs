use std::fmt::Debug;

use ratatui::widgets::Tabs;
use ratatui::prelude::*;
use anyhow::Result;

use crate::about_tab::AboutTab;
use crate::colors::BG_COLOR;
use crate::countdown_tab::CountdownTab;
use crate::msg::Message;
use crate::stopwatch_tab::StopwatchTab;

#[derive(Debug, Clone)]
pub struct TabGroup {
    current_tab_index: usize,
    titles: Vec<String>,
    tabs: Vec<Box<dyn AppTab>>,
}

impl TabGroup {
    pub fn new() -> Self {
        Self {
            current_tab_index: 0,
            titles: vec![
                "Stopwatch".to_string(),
                "Countdown".to_string(),
                "About".to_string()
            ],
            tabs: vec![
                Box::new(StopwatchTab::default()),
                Box::new(CountdownTab::default()),
                Box::new(AboutTab::default()),
            ]
        }
    }

    pub fn widget(&self) -> Tabs<'_> {
        let titles = self.titles.clone();
        Tabs::new(titles)
            .style(Style::default().white())
            .select(self.current_tab_index)
            .divider(" ")
            .padding(" ", " ")
            .bg(BG_COLOR)
    }

    pub fn increase_index(&mut self) {
        self.current_tab_index += 1;

        if self.current_tab_index >= self.titles.len() {
            self.current_tab_index = 0;
        }
    }

    pub fn current_tab(&mut self) -> &mut Box<dyn AppTab> {
        &mut self.tabs[self.current_tab_index]
    }
}

impl Default for TabGroup {
    fn default() -> Self {
        TabGroup::new()
    }
}

pub trait AppTab {
    fn handle_message(&mut self, message: Message);

    fn render_paragraph(&mut self, frame: &mut Frame, area: &Rect) -> Result<()>;

    fn help_text(&mut self) -> Vec<Span>;
}

impl Debug for dyn AppTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppTab_Debug")
    }
}

impl Clone for Box<dyn AppTab> {
    fn clone(&self) -> Self {
        Box::new(StopwatchTab::default())
    }
}