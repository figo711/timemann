use std::time::Duration;

use ratatui_splash_screen::{SplashConfig, SplashScreen};
use anyhow::{Ok, Result};

use crate::tui::Tui;

pub static SPLASH_CONFIG: SplashConfig = SplashConfig {
    image_data: include_bytes!("../assets/splash.png"),
    sha256sum: None,
    render_steps: 12,
    use_colors: true,
};


pub fn show_splash(tui: &mut Tui) -> Result<()> {
    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;

    while !splash_screen.is_rendered() {
        tui.draw(|frame| {
            frame.render_widget(&mut splash_screen, frame.size());
        })?;
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}