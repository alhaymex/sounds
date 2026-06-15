use anyhow::Result;

use super::{App, Screen};
use crate::state::input::InputTarget;
use crate::state::options::OptionsItem;
use crate::system::settings::Settings;
use crate::tui::help::HelpItem;

impl App {
    pub fn open_options(&mut self) {
        if self.screen == Screen::Options {
            return;
        }

        self.cancel_input();
        self.navigation_stack.push(self.screen.clone());
        self.screen = Screen::Options;
    }

    pub fn options_down(&mut self) {
        self.options.move_down();
    }

    pub fn options_up(&mut self) {
        self.options.move_up();
    }

    pub fn help_down(&mut self) {
        let len = self.help_len();

        if len == 0 {
            return;
        }

        if let Screen::Help { selected } = &mut self.screen {
            *selected = (*selected + 1) % len;
        }
    }

    pub fn help_up(&mut self) {
        let len = self.help_len();

        if len == 0 {
            return;
        }

        if let Screen::Help { selected } = &mut self.screen {
            *selected = if *selected == 0 {
                len - 1
            } else {
                *selected - 1
            };
        }
    }

    pub fn toggle_help(&mut self) {
        self.cancel_input();

        if matches!(self.screen, Screen::Help { .. }) {
            if let Some(prev) = self.navigation_stack.pop() {
                self.screen = prev;
            }
        } else {
            self.navigation_stack.push(self.screen.clone());
            self.screen = Screen::Help { selected: 0 };
        }
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

    fn help_len(&self) -> usize {
        self.help_items
            .iter()
            .filter(|item| matches!(item, HelpItem::Entry(_)))
            .count()
    }
}
