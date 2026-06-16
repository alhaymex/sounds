use std::fs;
use std::time::SystemTime;
use std::{io, path::Path};

use super::{
    metadata::read_metadata,
    model::{Library, Node, SongRef},
};
use crate::system::fs::{dir_name, is_audio_file, song_title_from_path};

pub fn scan_library(root: impl AsRef<Path>) -> io::Result<Library> {
    let root = root.as_ref().to_path_buf();
    let mut index = Vec::new();

    let tree = scan_dir(&root, &mut index)?;

    Ok(Library {
        root,
        tree: Some(tree),
        index,
    })
}

pub fn scan_dir(dir: &Path, index: &mut Vec<SongRef>) -> io::Result<Node> {
    let mut children = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
            return Ok(Node::Dir {
                name: dir_name(dir),
                path: dir.to_path_buf(),
                children,
            });
        }
        Err(error) => return Err(error),
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            match scan_dir(&path, index) {
                Ok(child) => children.push(child),
                Err(error)
                    if error.kind() == io::ErrorKind::PermissionDenied =>
                {
                    continue;
                }
                Err(error) => return Err(error),
            }
        } else if is_audio_file(&path) {
            let id = index.len();
            let modified = entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            let metadata = read_metadata(&path);
            let title = metadata
                .title
                .clone()
                .unwrap_or_else(|| song_title_from_path(&path));

            index.push(SongRef {
                id,
                path: path.clone(),
                title,
                metadata,
                modified,
            });

            children.push(Node::Song { id });
        }
    }

    Ok(Node::Dir {
        name: dir_name(dir),
        path: dir.to_path_buf(),
        children,
    })
}

pub fn find_dir_by_path<'a>(node: &'a Node, target: &Path) -> Option<&'a Node> {
    match node {
        Node::Dir { children, path, .. } => {
            if path == target {
                return Some(node);
            }

            for child in children {
                if let Some(found) = find_dir_by_path(child, target) {
                    return Some(found);
                }
            }

            None
        }
        Node::Song { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::model::Node;
    use std::fs::{File, create_dir_all};
    use std::io;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn scan_library_builds_index_from_audio_files() -> io::Result<()> {
        let temp = tempdir()?;
        let root = temp.path();

        File::create(root.join("song.mp3"))?;
        File::create(root.join("cover.jpg"))?;
        File::create(root.join("notes.txt"))?;

        let library = scan_library(root)?;

        assert_eq!(library.root, root.to_path_buf());
        assert_eq!(library.index.len(), 1);

        let song = &library.index[0];
        assert_eq!(song.id, 0);
        assert_eq!(song.title, "song");
        assert_eq!(song.path, root.join("song.mp3"));

        Ok(())
    }

    #[test]
    fn scan_library_scans_nested_directories() -> io::Result<()> {
        let temp = tempdir()?;
        let root = temp.path();

        create_dir_all(root.join("Artist/Album"))?;

        File::create(root.join("root-song.mp3"))?;
        File::create(root.join("Artist/Album/nested-song.flac"))?;
        File::create(root.join("Artist/Album/cover.png"))?;

        let library = scan_library(root)?;

        assert_eq!(library.index.len(), 2);

        let titles: Vec<&str> = library
            .index
            .iter()
            .map(|song| song.title.as_str())
            .collect();

        assert!(titles.contains(&"root-song"));
        assert!(titles.contains(&"nested-song"));

        let tree = library.tree.as_ref().expect("library should have a tree");
        assert_eq!(count_song_nodes(tree), 2);

        Ok(())
    }

    #[test]
    fn scan_library_returns_error_for_missing_directory() {
        let temp = tempdir().expect("failed to create temp dir");
        let missing_dir = temp.path().join("missing");

        let error = scan_library(&missing_dir).expect_err("scan should fail");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn find_dir_by_path_finds_nested_directory() {
        let root_path = PathBuf::from("/music");
        let artist_path = root_path.join("Artist");
        let album_path = artist_path.join("Album");

        let tree = Node::Dir {
            name: "Music".to_string(),
            path: root_path,
            children: vec![Node::Dir {
                name: "Artist".to_string(),
                path: artist_path,
                children: vec![Node::Dir {
                    name: "Album".to_string(),
                    path: album_path.clone(),
                    children: vec![Node::Song { id: 0 }],
                }],
            }],
        };

        let found = find_dir_by_path(&tree, &album_path)
            .expect("expected to find nested album directory");

        match found {
            Node::Dir { name, path, .. } => {
                assert_eq!(name, "Album");
                assert_eq!(path, &album_path);
            }
            Node::Song { .. } => {
                panic!("expected directory, found song");
            }
        }
    }

    #[test]
    fn find_dir_by_path_returns_none_for_missing_directory() {
        let root_path = PathBuf::from("/music");

        let tree = Node::Dir {
            name: "Music".to_string(),
            path: root_path,
            children: vec![Node::Dir {
                name: "Artist".to_string(),
                path: PathBuf::from("/music/Artist"),
                children: vec![],
            }],
        };

        let missing = PathBuf::from("/music/Missing");

        assert!(find_dir_by_path(&tree, &missing).is_none());
    }

    #[test]
    fn find_dir_by_path_does_not_match_songs() {
        let song_path = PathBuf::from("/music/song.mp3");

        let tree = Node::Dir {
            name: "Music".to_string(),
            path: PathBuf::from("/music"),
            children: vec![Node::Song { id: 0 }],
        };

        assert!(find_dir_by_path(&tree, &song_path).is_none());
    }

    #[cfg(unix)]
    #[test]
    fn scan_library_skips_permission_denied_directories() -> io::Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempdir()?;
        let root = temp.path();

        let readable_dir = root.join("readable");
        let unreadable_dir = root.join("unreadable");

        create_dir_all(&readable_dir)?;
        create_dir_all(&unreadable_dir)?;

        File::create(readable_dir.join("song.mp3"))?;

        let original_permissions = fs::metadata(&unreadable_dir)?.permissions();

        fs::set_permissions(
            &unreadable_dir,
            fs::Permissions::from_mode(0o000),
        )?;

        let result = scan_library(root);

        // Restore permissions before assertions/cleanup so tempdir can be deleted.
        fs::set_permissions(&unreadable_dir, original_permissions)?;

        let library = result?;

        assert_eq!(library.index.len(), 1);
        assert_eq!(library.index[0].title, "song");

        Ok(())
    }

    fn count_song_nodes(node: &Node) -> usize {
        match node {
            Node::Dir { children, .. } => {
                children.iter().map(count_song_nodes).sum()
            }
            Node::Song { .. } => 1,
        }
    }
}
