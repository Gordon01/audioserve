use unicase::UniCase;
use std::path::{PathBuf};

pub struct Artist {
    pub albums: i32,
    pub tracks: i32,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MediaLibrary {
    #[serde(with = "unicase_serde::unicase")]
    pub name: UniCase<String>,
    pub path: PathBuf,
}