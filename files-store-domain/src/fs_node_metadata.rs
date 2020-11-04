use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum FsNodeMetadata {
    File {
        stem: Option<String>,
        extension: Option<String>,
        hash: String,
        content_type: String,
        size: i64,
    },
    Directory,
    Root,
    Bin,
    Thumbnail {
        content_type: String,
        size: i64,
        width: i64,
        heigth: i64,
    },
}

pub fn get_filename_component(filename: &str) -> (Option<String>, Option<String>) {
    let path = std::path::Path::new(&filename);
    let stem = path.file_stem().map(|e| e.to_string_lossy().into_owned());
    let extension = path.extension().map(|e| e.to_string_lossy().into_owned());
    (stem, extension)
}

impl FsNodeMetadata {
    pub fn new_file(filename: &str, hash: String, content_type: String, size: i64) -> Self {
        let (stem, extension) = get_filename_component(&filename);
        Self::File {
            extension,
            stem,
            hash,
            content_type,
            size,
        }
    }

    pub fn new_thumbnail(content_type: String, size: i64, width: i64, heigth: i64) -> Self {
        Self::Thumbnail {
            content_type,
            size,
            width,
            heigth,
        }
    }
}
