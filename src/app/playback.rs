use std::path::PathBuf;
use std::time::Duration;

use super::{App, DirEntry, PlaybackStatus, Screen, SongId};
use crate::audio::player::SPEEDS;

impl App {
    pub fn advance_to_song(&mut self, song_id: SongId) -> Option<PathBuf> {
        let path = self.library.index.get(song_id)?.path.clone();

        self.playback.current_song = Some(song_id);
        self.playback.selected_song = Some(song_id);
        self.playback.status = PlaybackStatus::Playing;
        self.playback.position = Duration::ZERO;
        self.playback.duration = None;

        // Find the new cursor position
        let new_idx = match &self.screen {
            Screen::Library { path, .. } => {
                let dir_path = path.clone();
                let entries = self.dir_entries(&dir_path);
                entries.iter().position(
                    |e| matches!(e, DirEntry::Song { id, .. } if *id == song_id),
                )
            }
            Screen::Favorites { .. } => self
                .favorite_songs()
                .iter()
                .position(|song| song.id == song_id),
            Screen::Search { query, .. } => {
                self.search_results(query)
                    .iter()
                    .position(|song| song.id == song_id)
            }
            _ => None,
        };

        // update cursor
        if let Some(idx) = new_idx {
            match &mut self.screen {
                Screen::Library { selected, .. } => *selected = idx,
                Screen::Favorites { selected } => *selected = idx,
                Screen::Search { selected, .. } => *selected = idx,
                _ => {}
            }
        }

        Some(path)
    }

    pub fn mark_selected_song_playing(&mut self) {
        if let Some(song_id) = self.selected_song_id() {
            self.playback.selected_song = Some(song_id);
            self.playback.current_song = Some(song_id);
            self.playback.status = PlaybackStatus::Playing;
        }
    }

    pub fn sync_playback_time(
        &mut self,
        position: Duration,
        duration: Option<Duration>,
    ) {
        let speed = SPEEDS[self.playback.speed_index];
        self.playback.position = position.mul_f32(speed);
        self.playback.duration = duration;
    }

    pub fn sync_speed(&mut self, speed_index: usize) {
        self.playback.speed_index = speed_index;
    }

    pub fn stop_playback(&mut self) {
        self.playback.current_song = None;
        self.playback.status = PlaybackStatus::Stopped;
        self.playback.position = Duration::ZERO;
        self.playback.duration = None;
    }

    pub fn toggle_pause(&mut self) {
        self.playback.status = match self.playback.status {
            PlaybackStatus::Playing => PlaybackStatus::Paused,
            PlaybackStatus::Paused => PlaybackStatus::Playing,
            PlaybackStatus::Stopped => PlaybackStatus::Stopped,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.playback.volume = volume.min(100);
    }

    pub fn volume_up(&mut self) -> u8 {
        let volume = self.playback.volume.saturating_add(5).min(100);
        self.set_volume(volume);
        volume
    }

    pub fn volume_down(&mut self) -> u8 {
        let volume = self.playback.volume.saturating_sub(5);
        self.set_volume(volume);
        volume
    }
}
