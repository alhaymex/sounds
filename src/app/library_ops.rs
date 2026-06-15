use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::library::scan::scan_library;
use crate::system::settings::Settings;
use super::{App, Screen};

impl App {
    pub fn set_library_dir(&mut self, dir: PathBuf) -> Result<()> {
        if dir.exists() && !dir.is_dir() {
            bail!("library path is not a directory {}", dir.display());
        }

        if !dir.exists() {
            fs::create_dir_all(&dir).with_context(|| {
                format!("failed to create library directory {}", dir.display())
            })?;
        }

        let library = scan_library(&dir).with_context(|| {
            format!("failed to scan directory {}", dir.display())
        })?;

        self.library = library;
        self.navigation_stack.clear();
        self.screen = Screen::Library {
            path: dir.clone(),
            selected: 0,
        };

        let mut settings = Settings::load()?;
        settings.library_dir = Some(dir);

        settings.save()?;

        Ok(())
    }

    pub fn rename_entry(
        &mut self,
        original: &Path,
        new_name: &str,
    ) -> Result<()> {
        let new_name = new_name.trim();

        let file_name = if original.is_file() {
            match original.extension() {
                Some(ext) => format!("{}.{}", new_name, ext.to_string_lossy()),
                None => new_name.to_string(),
            }
        } else {
            new_name.to_string()
        };

        let new_path = original.parent().unwrap_or(original).join(file_name);

        fs::rename(original, &new_path).with_context(|| {
            format!("failed to rename {}", original.display())
        })?;

        self.rescan_library()
    }

    pub fn create_playlist(&mut self, parent: &Path, name: &str) -> Result<()> {
        let dir_path = parent.join(name.trim());

        fs::create_dir_all(&dir_path).with_context(|| {
            format!("failed to create playlist {}", dir_path.display())
        })?;

        self.rescan_library()
    }

    pub fn delete_entry(&mut self, path: &Path) -> Result<()> {
        if path.is_dir() {
            fs::remove_dir_all(path).with_context(|| {
                format!("failed to delete directory {}", path.display())
            })?;
        } else {
            fs::remove_file(path).with_context(|| {
                format!("failed to delete file {}", path.display())
            })?;
        }

        self.rescan_library()
    }

    pub fn rescan_library(&mut self) -> Result<()> {
        let library = scan_library(&self.library.root).with_context(|| {
            format!("failed to rescan library {}", self.library.root.display())
        })?;
        self.library = library;

        let new_selected =
            if let Screen::Library { ref path, selected } = self.screen {
                let path = path.clone();
                let count = self.dir_entries(&path).len();
                if selected >= count && count > 0 {
                    Some(count - 1)
                } else {
                    None
                }
            } else {
                None
            };

        if let (Some(new), Screen::Library { selected, .. }) =
            (new_selected, &mut self.screen)
        {
            *selected = new;
        }

        Ok(())
    }
}
