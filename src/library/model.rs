use std::{path::PathBuf, time::SystemTime};

pub type SongId = usize;

#[derive(Debug, Clone)]
pub enum Node {
    Dir {
        name: String,
        path: PathBuf,
        children: Vec<Node>,
    },
    Song {
        id: SongId,
    },
}

#[derive(Debug, Clone)]
pub struct SongRef {
    pub id: SongId,
    pub path: PathBuf,
    pub title: String,
    pub metadata: Metadata,
    pub modified: SystemTime,
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Library {
    pub root: PathBuf,

    /// Cached tree for UI rendering
    pub tree: Option<Node>,

    /// Flat index for fast playback/search
    pub index: Vec<SongRef>,
}
