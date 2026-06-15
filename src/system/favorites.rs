use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::system::fs::config_dir;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Favorites {
    pub songs: HashSet<PathBuf>,
}

impl Favorites {
    pub fn load() -> Result<Self> {
        Self::load_from(favorites_path()?)
    }

    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let favorites: Self = toml::from_str(&contents)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        Ok(favorites)
    }

    pub fn save(&self) -> Result<()> {
        Self::save_to(&self, favorites_path()?)
    }

    pub fn save_to(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create {}", parent.display())
            })?;
        }

        let contents = toml::to_string_pretty(self)
            .context("failed to serialize favorites")?;

        fs::write(path, contents)
            .with_context(|| format!("failed to write {}", path.display()))?;

        Ok(())
    }

    pub fn toggle_favorite(&mut self, path: &Path) -> bool {
        let path = path.to_path_buf();
        if self.songs.contains(&path) {
            self.songs.remove(&path);
            false
        } else {
            self.songs.insert(path);
            true
        }
    }
}

pub fn favorites_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("favorites.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_load_from_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("does_not_exist.toml");

        let favorites =
            Favorites::load_from(&file_path).expect("Failed to load favorites");

        assert!(favorites.songs.is_empty());
    }

    #[test]
    fn test_save_and_load_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("favorites.toml");

        let mut original_favorites = Favorites::default();
        original_favorites
            .songs
            .insert(PathBuf::from("/music/song_one.mp3"));
        original_favorites
            .songs
            .insert(PathBuf::from("/music/song_two.flac"));
        original_favorites
            .save_to(&file_path)
            .expect("Failed to save favorites");

        assert!(file_path.exists());

        let loaded_favorites =
            Favorites::load_from(&file_path).expect("Failed to load favorites");

        assert_eq!(original_favorites.songs, loaded_favorites.songs);
    }

    #[test]
    fn test_load_invalid_toml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("broken_favorites.toml");

        fs::write(&file_path, "songs = [\"missing_closing_bracket").unwrap();

        let result = Favorites::load_from(&file_path);

        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("failed to parse"));
    }

    #[test]
    fn test_empty_save_creates_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty_favorites.toml");

        let empty_favorites = Favorites::default();

        empty_favorites
            .save_to(&file_path)
            .expect("Failed to save empty favorites");

        assert!(file_path.exists());

        let loaded = Favorites::load_from(&file_path)
            .expect("Failed to load empty favorites");
        assert!(loaded.songs.is_empty());
    }
}
