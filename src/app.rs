use std::path::PathBuf;

use crate::{
    library::{
        self,
        model::Library,
        scan::{self, scan_library},
    },
    system::settings::{Settings, default_library_dir},
};

use anyhow::Result;

pub enum Screen {
    Library,
}

pub struct App {
    screen: Screen,
    library: Library,
}

impl App {
    pub fn new(&self) -> Result<Self> {
        let settings = Settings::load()?;

        let library_dir = settings
            .library_dir
            .or_else(default_library_dir)
            .unwrap_or_else(|| PathBuf::from("."));

        let library = scan_library(&library_dir)?;

        Ok(Self {
            screen: Screen::Library,
            library,
        })
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

    fn library(&self) -> &Library {
        &self.library
    }

    fn screen(&self) -> &Screen {
        &self.screen
    }
}
