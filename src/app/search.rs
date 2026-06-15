use std::path::PathBuf;

use super::{App, Screen, SongId};
use crate::library::model::SongRef;

impl App {
    pub fn search_results(&self, query: &str) -> Vec<&SongRef> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return self.library.index.iter().collect();
        }

        self.library
            .index
            .iter()
            .filter(|song| {
                song.title.to_lowercase().contains(&query)
                    || song
                        .metadata
                        .artist
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false)
                    || song
                        .metadata
                        .album
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false)
                    || song
                        .path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false)
            })
            .collect()
    }

    pub fn clamp_search_selection(&mut self) {
        let Screen::Search { query, .. } = &self.screen else {
            return;
        };

        let count = self.search_results(query).len();
        let max = count.saturating_sub(1);

        if let Screen::Search { selected, .. } = &mut self.screen {
            *selected = (*selected).min(max);
        }
    }

    pub fn search_down(&mut self) {
        let len = self.search_results_for_screen().len();
        if len == 0 {
            return;
        }

        if let Screen::Search { selected, .. } = &mut self.screen {
            *selected = (*selected + 1) % len;
        }
    }

    pub fn search_up(&mut self) {
        let len = self.search_results_for_screen().len();
        if len == 0 {
            return;
        }

        if let Screen::Search { selected, .. } = &mut self.screen {
            *selected = if *selected == 0 {
                len - 1
            } else {
                *selected - 1
            };
        }
    }

    fn search_results_for_screen(&self) -> Vec<&SongRef> {
        match &self.screen {
            Screen::Search { query, .. } => self.search_results(query),
            _ => Vec::new(),
        }
    }

    pub fn selected_search_song_id(&self) -> Option<SongId> {
        let Screen::Search { query, selected } = &self.screen else {
            return None;
        };

        self.search_results(query)
            .get(*selected)
            .map(|song| song.id)
    }

    pub fn selected_search_song_path(&self) -> Option<PathBuf> {
        self.selected_search_song_id()
            .and_then(|id| self.library.index.get(id))
            .map(|song| song.path.clone())
    }

    pub fn play_selected_search_result(&mut self) -> Option<PathBuf> {
        let song_id = self.selected_search_song_id()?;
        self.advance_to_song(song_id)
    }

    pub fn next_search_song_id(&self) -> Option<SongId> {
        let songs = self.search_results_for_screen();
        if songs.is_empty() {
            return None;
        }

        let current_id = self.playback.current_song?;
        let current_idx =
            songs.iter().position(|song| song.id == current_id)?;

        songs.get(current_idx + 1).map(|song| song.id)
    }

    pub fn prev_search_song_id(&self) -> Option<SongId> {
        let songs = self.search_results_for_screen();
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
