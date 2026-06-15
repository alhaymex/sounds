use std::path::PathBuf;
use std::time::Duration;

pub use crate::library::model::SongId;
pub use crate::tui::help::HelpItem;
pub use crate::tui::toast::Toast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Library { path: PathBuf, selected: usize },
    Options,
    Favorites,
    Help { selected: usize },
}

pub enum ConfirmAction {
    Delete { path: PathBuf, name: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Stopped,
    Playing,
    Paused,
}

pub struct PlaybackState {
    pub selected_song: Option<SongId>,
    pub current_song: Option<SongId>,
    pub status: PlaybackStatus,
    pub position: Duration,
    pub duration: Option<Duration>,
    pub volume: u8,
    pub speed_index: usize,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            selected_song: None,
            current_song: None,
            status: PlaybackStatus::Stopped,
            volume: 75,
            position: Duration::ZERO,
            duration: None,
            speed_index: 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirEntry {
    Dir {
        name: String,
        path: PathBuf,
        song_count: usize,
    },
    Song {
        id: SongId,
        title: String,
    },
}

pub struct Config {
    pub rain_enabled: bool,
}
