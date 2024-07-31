use anyhow::Result;

mod ui;
mod tui;
mod eve;
mod msg;
mod fps;
mod app;
mod splash;
mod tabs;
mod colors;
mod states;

mod timer;

mod stopwatch_tab;
mod countdown_tab;
mod about_tab;

use app::App;


#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::default();
    app.run().await
}