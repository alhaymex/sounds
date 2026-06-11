use anyhow::{Context, Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::state::{
    input::{InputTarget, TextInputState},
    options::{OptionsItem, OptionsState},
};
use crate::{
    library::{
        model::{Library, Node, SongId},
        scan::{find_dir_by_path, scan_library},
    },
    system::settings::{Settings, default_library_dir},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Library { path: PathBuf, selected: usize },
    Options,
    Help,
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
    pub volume: u8,
    pub position: Duration,
    pub duration: Option<Duration>,
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

pub struct App {
    pub screen: Screen,
    pub navigation_stack: Vec<Screen>,
    pub library: Library,
    pub should_quit: bool,
    pub playback: PlaybackState,
    pub tick: u64,
    pub config: Config,
    pub options: OptionsState,
    pub input: Option<TextInputState>,
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
            screen: Screen::Library {
                path: library_dir,
                selected: 0,
            },
            navigation_stack: Vec::new(),
            playback: PlaybackState::default(),
            should_quit: false,
            tick: 0,
            input: None,
            options: { OptionsState::default() },
            config: {
                Config {
                    rain_enabled: settings.rain_enabled,
                }
            },
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_up(&mut self) {
        let Screen::Library { ref path, selected } = self.screen else {
            return;
        };

        let path = path.clone();
        let selected = selected;
        let count = self.dir_entries(&path).len();
        if count == 0 {
            return;
        }

        let new = if selected == 0 {
            count - 1
        } else {
            selected - 1
        };

        if let Screen::Library {
            ref mut selected, ..
        } = self.screen
        {
            *selected = new;
        }
    }

    pub fn move_down(&mut self) {
        let Screen::Library { ref path, selected } = self.screen else {
            return;
        };

        let path = path.clone();
        let selected = selected;
        let count = self.dir_entries(&path).len();
        if count == 0 {
            return;
        }

        let new = (selected + 1) % count;

        if let Screen::Library {
            ref mut selected, ..
        } = self.screen
        {
            *selected = new;
        }
    }

    pub fn enter(&mut self) {
        let Screen::Library { ref path, selected } = self.screen else {
            return;
        };

        let path = path.clone();
        let entries = self.dir_entries(&path);
        let Some(entry) = entries.get(selected) else {
            return;
        };

        match entry {
            DirEntry::Dir {
                path: child_path, ..
            } => {
                let current = self.screen.clone();

                self.navigation_stack.push(current);
                self.screen = Screen::Library {
                    path: child_path.clone(),
                    selected: 0,
                };
            }
            DirEntry::Song { .. } => {}
        }
    }

    pub fn back(&mut self) {
        match self.screen {
            Screen::Library { .. } => {
                if let Some(prev) = self.navigation_stack.pop() {
                    self.screen = prev;
                }
            }
            Screen::Options | Screen::Help => {
                if let Some(prev) = self.navigation_stack.pop() {
                    self.screen = prev;
                } else {
                    self.screen = Screen::Library {
                        path: self.library.root.clone(),
                        selected: 0,
                    };
                }
            }
        }
    }

    pub fn current_playing_song_ids(&self) -> Vec<SongId> {
        let Screen::Library { ref path, .. } = self.screen else {
            return Vec::new();
        };

        let Some(root) = self.library.tree.as_ref() else {
            return Vec::new();
        };

        let Some(node) = find_dir_by_path(root, path) else {
            return Vec::new();
        };

        let mut songs = Vec::new();
        collect_songs(node, &mut songs);
        songs
    }

    pub fn selected_song_id(&self) -> Option<SongId> {
        let Screen::Library { ref path, selected } = self.screen else {
            return None;
        };

        let path = path.clone();
        let entries = self.dir_entries(&path);
        match entries.get(selected)? {
            DirEntry::Song { id, .. } => Some(*id),
            DirEntry::Dir { .. } => None,
        }
    }

    pub fn selected_song_path(&self) -> Option<&Path> {
        let song_id = self.selected_song_id()?;

        self.library
            .index
            .get(song_id)
            .map(|song| song.path.as_path())
    }

    pub fn next_song_id(&self) -> Option<SongId> {
        let current_id = self.playback.current_song?;

        let Screen::Library { ref path, .. } = self.screen else {
            return None;
        };

        let path = path.clone();
        let entries = self.dir_entries(&path);

        let current_idx = entries.iter().position(|entry| {
            matches!(entry, DirEntry::Song { id, .. } if *id == current_id)
        })?;

        entries[current_idx + 1..]
            .iter()
            .find_map(|entry| match entry {
                DirEntry::Song { id, .. } => Some(*id),
                DirEntry::Dir { .. } => None,
            })
    }

    pub fn advance_to_song(&mut self, song_id: SongId) -> Option<PathBuf> {
        let path = self.library.index.get(song_id)?.path.clone();

        self.playback.current_song = Some(song_id);
        self.playback.selected_song = Some(song_id);
        self.playback.status = PlaybackStatus::Playing;
        self.playback.position = Duration::ZERO;
        self.playback.duration = None;

        // Find the new cursor position
        let new_idx = if let Screen::Library { ref path, .. } = self.screen {
            let dir_path = path.clone();
            let entries = self.dir_entries(&dir_path);
            entries.iter().position(
                |e| matches!(e, DirEntry::Song { id, .. } if *id == song_id),
            )
        } else {
            None
        };

        // update cursor
        if let (Some(idx), Screen::Library { selected, .. }) =
            (new_idx, &mut self.screen)
        {
            *selected = idx;
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
        self.playback.position = position;
        self.playback.duration = duration;
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

    pub fn open_options(&mut self) {
        if self.screen == Screen::Options {
            return;
        }

        self.navigation_stack.push(self.screen.clone());
        self.screen = Screen::Options;
    }

    pub fn options_down(&mut self) {
        self.options.move_down();
    }

    pub fn options_up(&mut self) {
        self.options.move_up();
    }

    pub fn activate_selected_option(&mut self) -> Result<()> {
        match self.options.selected_item() {
            OptionsItem::LibraryPath => {
                self.start_input(
                    InputTarget::LibraryPath,
                    self.library.root.display().to_string(),
                );

                Ok(())
            }

            OptionsItem::RainEnabled => self.toggle_rain(),
        }
    }

    pub fn toggle_rain(&mut self) -> Result<()> {
        self.config.rain_enabled = !self.config.rain_enabled;

        let mut settings = Settings::load()?;
        settings.rain_enabled = self.config.rain_enabled;
        settings.save()?;

        Ok(())
    }

    pub fn set_library_dir(&mut self, dir: PathBuf) -> Result<()> {
        if dir.exists() && !dir.is_dir() {
            bail!("library path is not a directory {}", dir.display());
        }

        if !dir.exists() {
            fs::create_dir_all(&dir).with_context(|| {
                format!("failed to create library directory {}", dir.display())
            })?;
        }

        let library = scan_library(&dir).with_context(|| {
            format!("failed to scan directory {}", dir.display())
        })?;

        self.library = library;
        self.navigation_stack.clear();
        self.screen = Screen::Library {
            path: dir.clone(),
            selected: 0,
        };

        let mut settings = Settings::load()?;
        settings.library_dir = Some(dir);

        settings.save()?;

        Ok(())
    }

    pub fn toggle_help(&mut self) {
        if self.screen == Screen::Help {
            if let Some(prev) = self.navigation_stack.pop() {
                self.screen = prev;
            }
        } else {
            self.navigation_stack.push(self.screen.clone());
            self.screen = Screen::Help;
        }
    }

    pub fn is_input_active(&self) -> bool {
        self.input.is_some()
    }

    pub fn start_input(
        &mut self,
        target: InputTarget,
        value: impl Into<String>,
    ) {
        self.input = Some(TextInputState::new(target, value))
    }

    pub fn cancel_input(&mut self) {
        self.input = None;
    }

    pub fn input_char(&mut self, ch: char) {
        if let Some(input) = self.input.as_mut() {
            input.push_char(ch);
        }
    }

    pub fn input_backspace(&mut self) {
        if let Some(input) = self.input.as_mut() {
            input.pop_char();
        }
    }

    pub fn submit_input(&mut self) -> Result<()> {
        let Some(input) = self.input.take() else {
            return Ok(());
        };

        match input.target {
            InputTarget::LibraryPath => {
                let dir = PathBuf::from(input.value.trim());
                self.set_library_dir(dir)?;
            }
            // TODO: search should be Search(SearchScope)
            // we might add searching in multiple screens
            InputTarget::Search => {}
        }

        Ok(())
    }

    pub fn dir_entries(&self, path: &Path) -> Vec<DirEntry> {
        let Some(root) = self.library.tree.as_ref() else {
            return Vec::new();
        };

        let Some(node) = find_dir_by_path(root, path) else {
            return Vec::new();
        };

        let Node::Dir { children, .. } = node else {
            return Vec::new();
        };

        let mut entries: Vec<DirEntry> = children
            .iter()
            .filter_map(|child| match child {
                Node::Dir { name, path, .. } => {
                    let song_count = count_songs(child);

                    if song_count > 0 {
                        Some(DirEntry::Dir {
                            name: name.clone(),
                            path: path.clone(),
                            song_count,
                        })
                    } else {
                        None
                    }
                }
                Node::Song { id } => {
                    let title = self
                        .library
                        .index
                        .get(*id)
                        .map(|s| s.title.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    Some(DirEntry::Song { id: *id, title })
                }
            })
            .collect::<Vec<_>>();

        entries.sort_by_key(|entry| match entry {
            DirEntry::Dir { .. } => 0,
            DirEntry::Song { .. } => 1,
        });

        entries
    }
}

fn collect_songs(node: &Node, songs: &mut Vec<SongId>) {
    match node {
        Node::Dir { children, .. } => {
            for child in children {
                collect_songs(child, songs);
            }
        }
        Node::Song { id } => {
            songs.push(*id);
        }
    }
}

fn count_songs(node: &Node) -> usize {
    match node {
        Node::Dir { children, .. } => children.iter().map(count_songs).sum(),
        Node::Song { .. } => 1,
    }
}
