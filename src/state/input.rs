use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum InputTarget {
    LibraryPath,
    Rename { original_path: PathBuf },
    NewPlaylist { parent_path: PathBuf },
    Search,
}

pub struct TextInputState {
    pub target: InputTarget,
    pub value: String,
}

impl TextInputState {
    pub fn new(target: InputTarget, value: impl Into<String>) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.value.pop();
    }
}
