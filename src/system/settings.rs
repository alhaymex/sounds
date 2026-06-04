use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub library_dir: Option<PathBuf>,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let path = settings_path()?;

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
        let path = settings_path()?;

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
    let config_dir =
        dirs::config_dir().context("could not determine config directory")?;
    Ok(config_dir.join("sounds").join("settings.json"))
}

pub fn default_library_dir() -> Option<PathBuf> {
    dirs::audio_dir()
}
