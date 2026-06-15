use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::system::fs::config_dir;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub library_dir: Option<PathBuf>,
    pub rain_enabled: bool,
}

impl Settings {
    pub fn load() -> Result<Self> {
        Self::load_from(settings_path()?)
    }

    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let settings = serde_json::from_str(&contents)
            .with_context(|| format!("failed to read {}", path.display()))?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        self.save_to(settings_path()?)
    }

    pub fn save_to(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create {}", parent.display())
            })?;
        }

        let contents = serde_json::to_string_pretty(self)
            .context("failed to serialize settings")?;

        fs::write(&path, contents)
            .with_context(|| format!("failed to write {}", path.display()))?;

        Ok(())
    }
}

pub fn settings_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("settings.json"))
}

pub fn default_library_dir() -> Option<PathBuf> {
    dirs::audio_dir()
        .or_else(|| dirs::home_dir().map(|home| home.join("Music")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_from_missing_file_returns_default_settings() -> Result<()> {
        let temp = tempdir()?;
        let path = temp.path().join("settings.json");

        let settings = Settings::load_from(&path)?;

        assert!(settings.library_dir.is_none());

        Ok(())
    }

    #[test]
    fn save_to_writes_settings_json() -> Result<()> {
        let temp = tempdir()?;
        let path = temp.path().join("settings.json");

        let music_dir = temp.path().join("Music");

        let settings = Settings {
            library_dir: Some(music_dir.clone()),
            rain_enabled: true,
        };

        settings.save_to(&path)?;

        let contents = fs::read_to_string(&path)?;

        assert!(contents.contains("library_dir"));

        Ok(())
    }

    #[test]
    fn saved_settings_can_be_loaded_again() -> Result<()> {
        let temp = tempdir()?;
        let path = temp.path().join("settings.json");

        let music_dir = temp.path().join("Music");

        let settings = Settings {
            library_dir: Some(music_dir.clone()),
            rain_enabled: true,
        };

        settings.save_to(&path)?;

        let loaded = Settings::load_from(&path)?;

        assert_eq!(loaded.library_dir, Some(music_dir));

        Ok(())
    }

    #[test]
    fn save_to_creates_parent_directories() -> Result<()> {
        let temp = tempdir()?;
        let path = temp.path().join("sounds/settings.json");

        let settings = Settings {
            library_dir: Some(temp.path().join("Music")),
            rain_enabled: true,
        };

        settings.save_to(&path)?;

        assert!(path.exists());

        Ok(())
    }
}
