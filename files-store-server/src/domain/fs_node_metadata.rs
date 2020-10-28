use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum FsNodeMetadata {
    File {
        hash: String,
        content_type: String,
        size: i64,
    },
    Directory,
    Bin,
    Thumbnail {
        content_type: String,
        size: i64,
        width: i64,
        heigth: i64,
    },
}

impl FsNodeMetadata {
    pub fn new_file(hash: String, content_type: String, size: i64) -> Self {
        Self::File {
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
