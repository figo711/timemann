use ratatui::widgets::Paragraph;
use anyhow::Result;
use ratatui::prelude::*;

use crate::fps::FpsCounter;
use crate::msg::Message;
use crate::splash::show_splash;
use crate::tabs::TabGroup;
use crate::ui::*;
use crate::tui::Tui;
use crate::eve::EventHandler;
use crate::colors::BG_COLOR;

#[derive(Debug, Clone)]
pub struct App {
    active: bool,
    fps_counter: FpsCounter,
    tabs_group: TabGroup,
}

impl App {
    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::init()?;
        let mut events = EventHandler::new(60.0);
        
        if true {
            show_splash(&mut tui)?;
        }

        while self.active {
            self.draw(&mut tui)?;
            let message = events.next().await?;
            self.handle_message(message)?;
        }
        Ok(())
    }

    fn handle_message(&mut self, message: Message) -> Result<()> {
        match message {
            Message::ToggleStartPause | Message::Clear | Message::SetNumber(_) | Message::Edit
                => self.tabs_group.current_tab().handle_message(message),
            Message::ChangeTab => self.next_tab(),
            Message::Tick => self.tick(),
            Message::Quit => self.quit(),
        }
        Ok(())
    }

    fn next_tab(&mut self) {
        self.tabs_group.increase_index();
    }

    fn tick(&mut self) {
        self.fps_counter.tick()
    }

    fn quit(&mut self) {
        self.active = false;
    }

    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let layout = layout(frame.size());
            frame.render_widget(self.title_paragraph(), layout[0]);
            frame.render_widget(self.tabs_group.widget(), layout[1]);
            frame.render_widget(self.fps_paragraph(), layout[2]);
            let _ = self.tabs_group.current_tab().render_paragraph(frame, &layout[3]);
            frame.render_widget(self.help_paragraph(), layout[4]);
        })
    }

    fn title_paragraph(&mut self) -> Paragraph<'_> {
        let title = "timemann";
        Paragraph::new(title).bold().bg(BG_COLOR)
    }

    fn fps_paragraph(&mut self) -> Paragraph<'_> {
        let fps = format!("{:.2} fps", self.fps_counter.fps);
        Paragraph::new(fps)
            .dim()
            .right_aligned()
            .bg(BG_COLOR)
    }

    fn help_paragraph(&mut self) -> Paragraph<'_> {
        let mut start_vec = vec![
            " Tab ".on_dark_gray().black().dim(),
            " Next Tab ".dim(),
        ];

        let end_vec = vec![
            " Q ".on_dark_gray().black().dim(),
            " Quit ".dim(),
        ];

        // tab help text
        let line1 = self.tabs_group.current_tab().help_text();
        start_vec.extend(line1);
        start_vec.extend(end_vec);

        let help_text = Line::from(start_vec);
        Paragraph::new(help_text)
            .gray()
            .centered()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            active: true,
            fps_counter: Default::default(),
            tabs_group: Default::default()
        }
    }
}