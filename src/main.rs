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
    /// Ceck app version
    Version,
    /// Update to the latest version
    Update,
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Some(Command::Version) => {
            println!(
                "sounds v{}\n\n\tA filesystem-first audio player for the terminal\n\thttps://github.com/alhaymex/sounds\n",
                env!("CARGO_PKG_VERSION")
            );
        }
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

// Only work for linux/macos
fn update() -> Result<()> {
    println!("Updating sounds...");

    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg("curl -sSL https://raw.githubusercontent.com/alhaymex/sounds/main/install.sh | bash")
        .status()?;

    if !status.success() {
        anyhow::bail!("Update failed");
    }

    Ok(())
}
