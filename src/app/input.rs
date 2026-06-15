use std::path::PathBuf;

use anyhow::Result;

use super::{App, ConfirmAction, Screen};
use crate::state::input::{InputTarget, TextInputState};

impl App {
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

    pub fn start_rename(&mut self) {
        let Some((path, name)) = self.selected_entry_info() else {
            return;
        };

        self.start_input(
            InputTarget::Rename {
                original_path: path,
            },
            name,
        );
    }

    pub fn start_new_playlist(&mut self) {
        let parent = match self.screen {
            Screen::Library { ref path, .. } => path.clone(),
            _ => self.library.root.clone(),
        };

        self.start_input(
            InputTarget::NewPlaylist {
                parent_path: parent,
            },
            "",
        );
    }

    pub fn start_delete(&mut self) {
        let Some((path, name)) = self.selected_entry_info() else {
            return;
        };

        self.confirm = Some(ConfirmAction::Delete { path, name });
    }

    pub fn start_search(&mut self) {
        if matches!(self.screen, Screen::Search { .. }) {
            return;
        }

        self.navigation_stack.push(self.screen.clone());
        self.screen = Screen::Search {
            query: String::new(),
            selected: 0,
        };

        self.start_input(InputTarget::Search, "");
    }

    pub fn cancel_confirm(&mut self) {
        self.confirm = None;
    }

    pub fn is_confirming(&self) -> bool {
        self.confirm.is_some()
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

    pub fn search_input_char(&mut self, ch: char) {
        if let Some(input) = self.input.as_mut() {
            input.push_char(ch);
        }

        if let Screen::Search { query, .. } = &mut self.screen {
            if let Some(input) = &self.input {
                *query = input.value.clone();
            }
        }

        self.clamp_search_selection();
    }

    pub fn search_input_backspace(&mut self) {
        if let Some(input) = self.input.as_mut() {
            input.pop_char();
        }

        if let Screen::Search { query, .. } = &mut self.screen {
            if let Some(input) = &self.input {
                *query = input.value.clone();
            }
        }

        self.clamp_search_selection();
    }

    pub fn clear_search_input(&mut self) {
        if let Some(input) = self.input.as_mut() {
            input.value.clear();
        }

        if let Screen::Search { query, selected } = &mut self.screen {
            query.clear();
            *selected = 0;
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
            InputTarget::Search => {}
            InputTarget::Rename { original_path } => {
                if !input.value.trim().is_empty() {
                    self.rename_entry(&original_path, &input.value)?;
                }
            }
            InputTarget::NewPlaylist { parent_path } => {
                if !input.value.trim().is_empty() {
                    self.create_playlist(&parent_path, &input.value)?;
                }
            }
        }

        Ok(())
    }
}
