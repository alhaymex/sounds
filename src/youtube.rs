use anyhow::{Context, Result, bail};
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn is_yt_dlp_available() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn install_instructions() -> &'static str {
    "yt-dlp is not installed.\n\nInstall it from: https://github.com/yt-dlp/yt-dlp#installation"
}

pub fn download_audio(url: &str, output_dir: &Path) -> Result<()> {
    if !is_yt_dlp_available() {
        bail!("{}", install_instructions());
    }

    let status = Command::new("yt-dlp")
        .args([
            "-x",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "--embed-metadata",
            "--no-playlist",
            "--output",
            "%(title)s.%(ext)s",
            url,
        ])
        .current_dir(output_dir)
        .status()
        .context("failed to run yt-dlp")?;

    if !status.success() {
        bail!("yt-dlp exited with an error");
    }

    Ok(())
}
