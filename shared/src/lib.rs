use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub enum FileType {
    ASCII,
    PNG,
    JPG,
    MP4,
    MP3,
    Unknown,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct FileMeta {
    pub file_name: String,
    pub file_size: i32,
    pub file_uploaded: i32,
    pub file_link: String,
    pub file_type: FileType,
}
