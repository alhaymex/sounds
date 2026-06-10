mod app;
mod audio;
mod event;
mod library;
mod state;
mod system;
mod tui;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use crate::app::App;

#[derive(Parser, Debug)]
#[command(
    name = "sounds",
    version,
    about = "A filesystem-first audio player for the terminal",
    long_about = "Sounds is a lightweight terminal audio player that lets you browse, manage, and play music directly from your local files."
)]
struct CLI {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Update to the latest version
    Update,
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Some(Command::Update) => {
            update()?;
        }
        None => {
            let app = App::new()?;
            tui::run(app)?;
        }
    }

    Ok(())
}

fn update() -> Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("alhaymex")
        .repo_name("sounds")
        .bin_name("sounds")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    println!("Updated sounds to {}", status.version());

    Ok(())
}
