mod app;
mod audio;
mod library;
mod system;
mod tui;

use anyhow::{Ok, Result};

use crate::app::App;

fn main() -> Result<()> {
    let app = App::new()?;

    println!("Scanning {}", app.library().root.display());
    println!("Loaded {} songs", app.library().index.len());

    Ok(())
}
