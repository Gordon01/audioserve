use std::io;
use std::path::{Path, PathBuf};

use crate::services::types::*;

pub fn is_lib_path(path: &Path) -> bool {
    path.to_str().unwrap().contains('|')
}

pub fn list_medialibrary(path: &Path,
    ordering: FoldersOrdering) -> Result<AudioFolder, io::Error> {
    let mut files = vec![];
    let mut subfolders = vec![];
    let mut cover = None;
    let mut description = None;

    subfolders.push(AudioFolderShort::from_label("Lib item 1"));
    subfolders.push(AudioFolderShort::from_label("Lib item 2"));
    subfolders.push(AudioFolderShort::from_label("Lib item 3"));
    
    Ok(AudioFolder {
        files,
        subfolders,
        cover,
        description,
    })
}

pub fn insert_mediadb(subfolders: &mut Vec<AudioFolderShort>) {
    subfolders.insert(0, AudioFolderShort::from_label("Artists"));
    subfolders.insert(1, AudioFolderShort::from_label("Albums"));
    subfolders.insert(2, AudioFolderShort::from_label("Tracks"));
    subfolders.insert(3, AudioFolderShort::from_label("Playlists"));
}
