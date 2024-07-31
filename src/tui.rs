use std::io::{self, Stdout};

use anyhow::{Context, Result};
use ratatui::prelude::*;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}};


pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn init() -> Result<Tui> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("failed to create terminal")?;
        enable_raw_mode().context("failed to enable raw mode")?;
        terminal.hide_cursor().context("failed to hide cursor")?;
        terminal.clear().context("failed to clear console")?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, frame: impl FnOnce(&mut Frame)) -> Result<()> {
        self.terminal.draw(frame).context("failed to draw frame")?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("failed to switch to main screen");
        self.terminal.show_cursor().expect("failed to show cursor");
        self.terminal.clear().expect("failed to clear console");
    }
}