mod app;
mod audio;
mod event;
mod library;
mod system;
mod tui;

use anyhow::{Ok, Result};

use crate::app::App;

fn main() -> Result<()> {
    let app = App::new()?;
    let lib = app.library().clone();
    tui::run(app)?;

    println!("{lib:?}");

    Ok(())
}
