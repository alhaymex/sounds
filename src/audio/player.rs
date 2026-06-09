use std::{
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Ok, Result};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

pub struct AudioPlayer {
    sink: MixerDeviceSink,
    player: Player,
    current_path: Option<PathBuf>,
    current_duration: Option<Duration>,
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
            current_duration: None,
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

        self.current_duration = source.total_duration();

        self.player.append(source);
        self.player.play();

        self.current_path = Some(path.to_path_buf());

        Ok(())
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.current_duration = None;

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

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn duration(&self) -> Option<Duration> {
        self.current_duration
    }

    pub fn seek_forward(&mut self) -> Result<()> {
        let position = self.player.get_pos();
        let target = position + Duration::from_secs(5);

        let target = if let Some(duration) = self.current_duration {
            let max = duration.saturating_sub(Duration::from_millis(200));
            target.min(max)
        } else {
            target
        };

        self.player
            .try_seek(target)
            .context("failed to seek forward")?;

        Ok(())
    }

    pub fn seek_backward(&mut self) -> Result<()> {
        let position = self.player.get_pos();
        let target =
            position.saturating_sub(Duration::from_secs(5).max(Duration::ZERO));

        self.player
            .try_seek(target)
            .context("failed to seek backward")?;

        Ok(())
    }

    pub fn set_volume(&self, volume: u8) {
        let volume = volume.min(100) as f32 / 100.0;
        self.player.set_volume(volume);
    }

    pub fn volume(&self) -> u8 {
        (self.player.volume() * 100.0).round() as u8
    }
}
