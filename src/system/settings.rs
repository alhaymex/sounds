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
}

pub fn settings_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().context("could not determine config directory")?;
    Ok(config_dir.join("sounds").join("settings.json"))
}
