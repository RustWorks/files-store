use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString, strum::ToString, Deserialize, Serialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum FsNodeType {
    File,
    Directory,
    Root,
    Bin,
    Thumbnail,
}
