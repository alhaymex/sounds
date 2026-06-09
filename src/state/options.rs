#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsItem {
    LibraryPath,
    RainEnabled,
}

#[derive(Debug, Default)]
pub struct OptionsState {
    selected: usize,
}

impl OptionsState {
    pub const ITEMS: [OptionsItem; 2] =
        [OptionsItem::LibraryPath, OptionsItem::RainEnabled];

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_item(&self) -> OptionsItem {
        Self::ITEMS[self.selected]
    }

    pub fn move_down(&mut self) {
        self.selected = (self.selected + 1) % Self::ITEMS.len();
    }

    pub fn move_up(&mut self) {
        if self.selected == 0 {
            self.selected = Self::ITEMS.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
