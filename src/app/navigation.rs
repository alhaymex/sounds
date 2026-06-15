use std::path::PathBuf;

use super::{App, DirEntry, Screen, SongId};
use crate::state::input::{InputTarget, TextInputState};

impl App {
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
            Screen::Options
            | Screen::Search { .. }
            | Screen::Favorites { .. }
            | Screen::Help { .. } => {
                if let Some(prev) = self.navigation_stack.pop() {
                    let query = match &prev {
                        Screen::Search { query, .. } => Some(query.clone()),
                        _ => None,
                    };

                    self.screen = prev;

                    if let Some(query) = query {
                        self.input = Some(TextInputState::new(
                            InputTarget::Search,
                            query,
                        ));
                    }
                } else {
                    self.screen = Screen::Library {
                        path: self.library.root.clone(),
                        selected: 0,
                    };
                }
            }
        }
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

    pub fn selected_song_path(&self) -> Option<PathBuf> {
        let song_id = self.selected_song_id()?;

        self.library
            .index
            .get(song_id)
            .map(|song| song.path.clone())
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

    pub fn prev_song_id(&self) -> Option<SongId> {
        let current_id = self.playback.current_song?;

        let Screen::Library { ref path, .. } = self.screen else {
            return None;
        };

        let path = path.clone();
        let entries = self.dir_entries(&path);

        let current_idx = entries.iter().position(|entry| {
            matches!(entry, DirEntry::Song { id, .. } if *id == current_id)
        })?;

        entries[..current_idx]
            .iter()
            .rev()
            .find_map(|entry| match entry {
                DirEntry::Song { id, .. } => Some(*id),
                DirEntry::Dir { .. } => None,
            })
    }

    pub fn selected_entry_info(&self) -> Option<(PathBuf, String)> {
        let Screen::Library { ref path, selected } = self.screen else {
            return None;
        };

        let path = path.clone();
        let entries = self.dir_entries(&path);
        let entry = entries.get(selected)?;

        match entry {
            DirEntry::Dir { name, path, .. } => {
                Some((path.clone(), name.clone()))
            }
            DirEntry::Song { id, title } => {
                let song_path = self.library.index.get(*id)?.path.clone();
                Some((song_path, title.clone()))
            }
        }
    }
}
