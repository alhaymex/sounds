use std::path::PathBuf;

use crate::{
    library::{
        model::{Library, SongId},
        scan::scan_library,
    },
    system::settings::{Settings, default_library_dir},
};

use anyhow::Result;

pub enum Screen {
    Library,
}

pub enum PlaybackStatus {
    Stopped,
}

pub struct PlaybackState {
    pub selected_song: Option<SongId>,
    pub current_song: Option<SongId>,
    pub status: PlaybackStatus,
    pub volume: u8,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            selected_song: None,
            current_song: None,
            status: PlaybackStatus::Stopped,
            volume: 100,
        }
    }
}

pub struct App {
    pub screen: Screen,
    pub library: Library,
    pub should_quit: bool,
    pub playback: PlaybackState,
}

impl App {
    pub fn new() -> Result<Self> {
        let settings = Settings::load()?;

        let library_dir = settings
            .library_dir
            .or_else(default_library_dir)
            .unwrap_or_else(|| PathBuf::from("."));

        let library = scan_library(&library_dir)?;

        Ok(Self {
            library,
            screen: Screen::Library,
            playback: PlaybackState::default(),
            should_quit: false,
        })
    }

    pub fn set_library_dir(&mut self, dir: PathBuf) -> Result<()> {
        let library = scan_library(&dir)?;

        self.library = library;

        Settings {
            library_dir: Some(dir),
        }
        .save()?;

        Ok(())
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }
}
