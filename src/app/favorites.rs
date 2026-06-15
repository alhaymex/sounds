use anyhow::Result;

use crate::library::model::SongRef;
use crate::tui::toast::Toast;
use super::{App, Screen};

impl App {
    pub fn open_favorites(&mut self) {
        if self.screen == Screen::Favorites {
            return;
        }

        self.favorites_selected = 0;
        self.navigation_stack.push(self.screen.clone());
        self.screen = Screen::Favorites;
    }

    pub fn add_to_favorites(&mut self) -> Result<()> {
        let Some(path) = self.selected_song_path() else {
            return Ok(());
        };

        let added = self.favorites.toggle_favorite(&path);
        self.favorites.save()?;

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

        self.favorites_selected = (self.favorites_selected + 1) % len;
    }

    pub fn favorites_up(&mut self) {
        let len = self.favorite_songs().len();
        if len == 0 {
            return;
        }

        self.favorites_selected = if self.favorites_selected == 0 {
            len - 1
        } else {
            self.favorites_selected - 1
        };
    }
}
