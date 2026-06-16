use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::library::{
    model::{Library, Node},
    scan::{find_dir_by_path, scan_library},
};
use crate::state::{input::TextInputState, options::OptionsState};
use crate::system::{
    favorites::Favorites,
    settings::{Settings, default_library_dir},
};
use crate::tui::help::help_items;

mod favorites;
mod input;
mod library_ops;
mod navigation;
mod playback;
mod screens;
mod search;
mod state;
mod toast;

pub use state::*;

pub struct App {
    pub(crate) screen: Screen,
    pub(crate) navigation_stack: Vec<Screen>,
    pub(crate) library: Library,
    pub(crate) favorites: Favorites,
    pub(crate) should_quit: bool,
    pub(crate) playback: PlaybackState,
    pub(crate) tick: u64,
    pub(crate) config: Config,
    pub(crate) options: OptionsState,
    pub(crate) input: Option<TextInputState>,
    pub(crate) confirm: Option<ConfirmAction>,
    pub(crate) help_items: Vec<HelpItem>,
    pub(crate) toasts: Vec<Toast>,
    toast_id_counter: u64,
}

impl App {
    pub fn new() -> Result<Self> {
        let settings = Settings::load()?;

        let library_dir = settings
            .library_dir
            .or_else(default_library_dir)
            .unwrap_or_else(|| PathBuf::from("."));

        let library = scan_library(&library_dir)?;

        let favorites = Favorites::load()?;

        Ok(Self {
            library,
            favorites,
            screen: Screen::Library {
                path: library_dir,
                selected: 0,
            },
            navigation_stack: Vec::new(),
            playback: PlaybackState::default(),
            should_quit: false,
            tick: 0,
            input: None,
            options: OptionsState::default(),
            config: Config {
                rain_enabled: settings.rain_enabled,
            },
            confirm: None,
            help_items: help_items(),
            toasts: Vec::new(),
            toast_id_counter: 0,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub(crate) fn dir_entries(&self, path: &Path) -> Vec<DirEntry> {
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

                    Some(DirEntry::Dir {
                        name: name.clone(),
                        path: path.clone(),
                        song_count,
                    })
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

        entries.sort_by(|a, b| {
            let a_kind = match a {
                DirEntry::Dir { .. } => 0,
                DirEntry::Song { .. } => 1,
            };
            let b_kind = match b {
                DirEntry::Dir { .. } => 0,
                DirEntry::Song { .. } => 1,
            };

            a_kind.cmp(&b_kind).then_with(|| {
                // If both are songs, compare by modified time (newest first), then title.
                let (a_id, a_title) = if let DirEntry::Song { id, title } = a {
                    (*id, title)
                } else {
                    return std::cmp::Ordering::Equal;
                };

                let (b_id, b_title) = if let DirEntry::Song { id, title } = b {
                    (*id, title)
                } else {
                    return std::cmp::Ordering::Equal;
                };

                let a_modified = self
                    .library
                    .index
                    .get(a_id)
                    .map(|s| s.modified)
                    .unwrap_or(SystemTime::UNIX_EPOCH);

                let b_modified = self
                    .library
                    .index
                    .get(b_id)
                    .map(|s| s.modified)
                    .unwrap_or(SystemTime::UNIX_EPOCH);

                b_modified.cmp(&a_modified).then(a_title.cmp(b_title))
            })
        });

        entries
    }
}

fn count_songs(node: &Node) -> usize {
    match node {
        Node::Dir { children, .. } => children.iter().map(count_songs).sum(),
        Node::Song { .. } => 1,
    }
}
