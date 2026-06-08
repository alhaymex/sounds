use std::path::PathBuf;

use anyhow::Result;

use crate::{
    library::{
        model::{Library, Node, SongId},
        scan::{find_dir_by_path, scan_library},
    },
    system::settings::{Settings, default_library_dir},
};

pub enum Screen {
    Library,
    Playlist { path: PathBuf },
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

pub struct PlaylistRow {
    pub name: String,
    pub path: PathBuf,
    pub song_count: usize,
}

pub struct App {
    pub screen: Screen,
    pub library: Library,
    pub should_quit: bool,
    pub selected_library: usize,
    pub selected_playlist: usize,
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
            selected_library: 0,
            selected_playlist: 0,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
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

    pub fn playlist_rows(&self) -> Vec<PlaylistRow> {
        let mut rows = Vec::new();

        let Some(root) = self.library.tree.as_ref() else {
            return rows;
        };

        match root {
            Node::Dir { children, .. } => {
                for child in children {
                    if matches!(child, Node::Dir { .. }) {
                        collect_playlist_rows(child, &mut rows);
                    }
                }
            }
            Node::Song { .. } => {}
        };

        rows
    }

    pub fn current_playing_song_ids(&self) -> Vec<SongId> {
        let Screen::Playlist { path } = &self.screen else {
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

    pub fn move_down(&mut self) {
        match self.screen {
            Screen::Library => {
                let rows_len = self.playlist_rows().len();

                if rows_len == 0 {
                    return;
                }

                self.selected_library = (self.selected_library + 1) % rows_len;
            }

            Screen::Playlist { .. } => {
                let row_len = self.current_playing_song_ids().len();

                if row_len == 0 {
                    return;
                }

                self.selected_playlist = (self.selected_playlist + 1) % row_len;
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.screen {
            Screen::Library => {
                let rows_len = self.playlist_rows().len();

                if rows_len == 0 {
                    return;
                }

                if self.selected_library == 0 {
                    self.selected_library = rows_len - 1;
                } else {
                    self.selected_library -= 1;
                }
            }

            Screen::Playlist { .. } => {
                let rows_len = self.current_playing_song_ids().len();

                if rows_len == 0 {
                    return;
                }

                if self.selected_playlist == 0 {
                    self.selected_playlist = rows_len - 1;
                } else {
                    self.selected_playlist -= 1;
                }
            }
        }
    }

    pub fn enter(&mut self) {
        match self.screen {
            Screen::Library => {
                let rows = self.playlist_rows();

                if let Some(row) = rows.get(self.selected_library) {
                    self.screen = Screen::Playlist {
                        path: row.path.clone(),
                    };

                    self.selected_playlist = 0;
                }
            }

            Screen::Playlist { .. } => {
                // Play selected song and go forward from there
            }
        }
    }

    pub fn back(&mut self) {
        match self.screen {
            Screen::Library => {}
            Screen::Playlist { .. } => {
                self.screen = Screen::Library;
            }
        }
    }
}

fn collect_playlist_rows(node: &Node, rows: &mut Vec<PlaylistRow>) {
    match node {
        Node::Dir {
            name,
            path,
            children,
        } => {
            let song_count = count_songs(node);

            if song_count > 0 {
                rows.push(PlaylistRow {
                    name: name.clone(),
                    path: path.clone(),
                    song_count: count_songs(node),
                });
            }

            for child in children {
                if matches!(child, Node::Dir { .. }) {
                    collect_playlist_rows(child, rows);
                }
            }
        }

        Node::Song { .. } => {}
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
