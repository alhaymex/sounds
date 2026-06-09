use std::path::Path;

use super::model::Metadata;
use lofty::{prelude::*, read_from_path};

pub fn read_metadata(path: &Path) -> Metadata {
    let Ok(tagged_file) = read_from_path(path) else {
        return Metadata::default();
    };

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let mut metadata = Metadata {
        ..Metadata::default()
    };

    if let Some(tag) = tag {
        metadata.title = tag.title().map(|value| value.into_owned());
        metadata.artist = tag.artist().map(|value| value.into_owned());
        metadata.album = tag.album().map(|value| value.into_owned());
    }

    metadata
}
