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

fn update() -> Result<()> {
    let (target, bin_path) = update_target();

    println!("Updating sounds...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("alhaymex")
        .repo_name("sounds")
        .bin_name("sounds")
        .target(target)
        .bin_path_in_archive(bin_path)
        .show_download_progress(true)
        .show_output(true)
        .no_confirm(false)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    match status {
        self_update::Status::Updated(version) => {
            println!("Updated to v{version}");
        }
        self_update::Status::UpToDate(version) => {
            println!("Already up to date (v{version})");
        }
    }

    Ok(())
}

fn update_target() -> (&'static str, &'static str) {
    if cfg!(target_os = "windows") {
        ("windows-x86_64", "sounds.exe")
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            ("macos-arm64", "sounds")
        } else {
            ("macos-x86_64", "sounds")
        }
    } else {
        ("linux-x86_64", "sounds")
    }
}
