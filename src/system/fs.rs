use std::path::Path;

// NOTE: webm files are not supported for now

pub fn is_audio_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("mp3" | "flac" | "wav" | "ogg")
    )
}

pub fn song_title_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string()
}

pub fn dir_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("/")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn detects_audio_files() {
        assert!(is_audio_file(Path::new("song.mp3")));
        assert!(is_audio_file(Path::new("song.flac")));
        assert!(is_audio_file(Path::new("song.wav")));
        assert!(is_audio_file(Path::new("song.ogg")));
    }

    #[test]
    fn detects_audio_files_case_insensitively() {
        assert!(is_audio_file(Path::new("song.MP3")));
        assert!(is_audio_file(Path::new("song.FLAC")));
    }

    #[test]
    fn rejects_non_audio_files() {
        assert!(!is_audio_file(Path::new("cover.jpg")));
        assert!(!is_audio_file(Path::new("notes.txt")));
        assert!(!is_audio_file(Path::new("song")));
    }

    #[test]
    fn gets_song_title_from_file_stem() {
        assert_eq!(
            song_title_from_path(Path::new("/music/artist/my-song.mp3")),
            "my-song"
        );
    }

    #[test]
    fn gets_dir_name_from_path() {
        assert_eq!(dir_name(Path::new("/home/user/Music")), "Music");
    }
}
