use std::{
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

pub const SPEEDS: &[f32] = &[0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

pub struct AudioPlayer {
    sink: MixerDeviceSink,
    player: Player,
    current_path: Option<PathBuf>,
    current_duration: Option<Duration>,
    volume: u8,
    speed_index: usize,
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
            volume: 75,
            speed_index: 2,
        })
    }

    pub fn play(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // Stop current song, but keep the same Player/audio device alive.
        self.player.stop();
        self.current_path = None;
        self.current_duration = None;

        let file = File::open(path).with_context(|| {
            format!("failed to open audio file {}", path.display())
        })?;

        let source = Decoder::try_from(file).with_context(|| {
            format!("failed to decode audio file {}", path.display())
        })?;

        self.current_duration = source.total_duration();

        self.player.append(source);

        // Re-apply persisted volume before playing.
        self.player.set_volume(self.volume_as_f32());
        self.player.set_speed(self.speed());

        self.player.play();

        self.current_path = Some(path.to_path_buf());

        Ok(())
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.current_duration = None;
        self.current_path = None;
    }

    pub fn toggle_pause(&mut self) {
        if self.player.is_paused() {
            self.player.play();
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

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
        self.player.set_volume(self.volume_as_f32());
    }

    pub fn _volume(&self) -> u8 {
        self.volume
    }

    pub fn is_finished(&self) -> bool {
        self.current_path.is_some() && self.player.empty()
    }

    pub fn speed(&self) -> f32 {
        SPEEDS[self.speed_index]
    }

    pub fn speed_index(&self) -> usize {
        self.speed_index
    }

    pub fn increase_speed(&mut self) {
        self.speed_index = (self.speed_index + 1).min(SPEEDS.len() - 1);
        self.player.set_speed(self.speed());
    }

    pub fn decrease_speed(&mut self) {
        self.speed_index = self.speed_index.saturating_sub(1);
        self.player.set_speed(self.speed());
    }

    fn volume_as_f32(&self) -> f32 {
        self.volume as f32 / 100.0
    }
}
