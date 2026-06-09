mod app;
mod audio;
mod event;
mod library;
mod state;
mod system;
mod tui;

use anyhow::{Ok, Result};

use crate::app::App;

fn main() -> Result<()> {
    let app = App::new()?;
    tui::run(app)?;

    Ok(())
}
