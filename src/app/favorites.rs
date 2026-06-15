use std::path::PathBuf;

use anyhow::Result;

use super::{App, Screen};
use crate::app::SongId;
use crate::library::model::SongRef;
use crate::tui::toast::Toast;

impl App {
    pub fn open_favorites(&mut self) {
        if matches!(self.screen, Screen::Favorites { .. }) {
            return;
        }

        self.navigation_stack.push(self.screen.clone());
        self.screen = Screen::Favorites { selected: 0 };
    }

    pub fn add_to_favorites(&mut self) -> Result<()> {
        let is_library = matches!(self.screen, Screen::Library { .. });
        let is_favorites = matches!(self.screen, Screen::Favorites { .. });

        let selected = match &self.screen {
            Screen::Favorites { selected } => *selected,
            _ => 0,
        };

        let path = if is_library {
            self.selected_song_path()
        } else if is_favorites {
            self.favorite_songs()
                .get(selected)
                .map(|song| song.path.clone())
        } else {
            None
        };

        let Some(path) = path else {
            return Ok(());
        };

        let added = self.favorites.toggle_favorite(&path);
        self.favorites.save()?;

        // Clamp Favorites selection after removal
        if is_favorites {
            let len = self.favorite_songs().len();
            if let Screen::Favorites { selected } = &mut self.screen {
                if len > 0 && *selected >= len {
                    *selected = len - 1;
                }
            }
        }

        self.push_toast(Toast::success(if added {
            "Added to favorites"
        } else {
            "Removed from favorites"
        }));

        Ok(())
    }

    pub fn favorite_songs(&self) -> Vec<&SongRef> {
        let mut songs: Vec<&SongRef> = self
            .favorites
            .songs
            .iter()
            .filter_map(|path| {
                self.library.index.iter().find(|song| song.path == *path)
            })
            .collect();

        songs.sort_by(|a, b| a.title.cmp(&b.title));
        songs
    }

    pub fn favorites_down(&mut self) {
        let len = self.favorite_songs().len();
        if len == 0 {
            return;
        }

        if let Screen::Favorites { selected } = &mut self.screen {
            *selected = (*selected + 1) % len;
        }
    }

    pub fn favorites_up(&mut self) {
        let len = self.favorite_songs().len();
        if len == 0 {
            return;
        }

        if let Screen::Favorites { selected } = &mut self.screen {
            *selected = if *selected == 0 {
                len - 1
            } else {
                *selected - 1
            };
        }
    }

    pub fn play_selected_favorite(&mut self) -> Option<PathBuf> {
        let selected = match &self.screen {
            Screen::Favorites { selected } => *selected,
            _ => return None,
        };

        let song_id =
            self.favorite_songs().get(selected).map(|song| song.id)?;

        self.advance_to_song(song_id)
    }

    pub fn next_favorite_song_id(&self) -> Option<SongId> {
        let songs = self.favorite_songs();
        if songs.is_empty() {
            return None;
        }

        let current_id = self.playback.current_song?;
        let current_idx =
            songs.iter().position(|song| song.id == current_id)?;

        songs.get(current_idx + 1).map(|song| song.id)
    }

    pub fn prev_favorite_song_id(&self) -> Option<SongId> {
        let songs = self.favorite_songs();
        if songs.is_empty() {
            return None;
        }

        let current_id = self.playback.current_song?;
        let current_idx =
            songs.iter().position(|song| song.id == current_id)?;

        if current_idx == 0 {
            None
        } else {
            songs.get(current_idx - 1).map(|song| song.id)
        }
    }
}
