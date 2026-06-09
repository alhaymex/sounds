use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

pub struct AudioPlayer {
    sink: MixerDeviceSink,
    player: Player,
    current_path: Option<PathBuf>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let sink = DeviceSinkBuilder::open_default_sink()
            .context("failed to open default device sink")?;

        let player = Player::connect_new(&sink.mixer());

        Ok(Self {
            sink,
            player,
            current_path: None,
        })
    }

    pub fn play(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        self.stop();

        let file = File::open(path).with_context(|| {
            format!("failed to open audio file {}", path.display())
        })?;

        let source = Decoder::try_from(file).with_context(|| {
            format!("failed to decode audio file {}", path.display())
        })?;

        self.player.append(source);
        self.player.play();

        self.current_path = Some(path.to_path_buf());

        Ok(())
    }

    pub fn stop(&mut self) {
        self.player.stop();

        // Create a new player after stopping so the next song starts cleanly
        self.player = Player::connect_new(&self.sink.mixer());
        self.current_path = None;
    }

    pub fn toggle_pause(&mut self) {
        if self.player.is_paused() {
            self.player.play()
        } else {
            self.player.pause();
        }
    }
}
