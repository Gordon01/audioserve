use std::io;
use std::path::{Path};
use super::artists::{get_artists, get_artist};

use crate::services::types::*;

enum PathType {
    Artist,
    Album,
    Track,
    Playlist,
}

pub fn is_lib_path(path: &Path) -> bool {
    path.to_str().unwrap().contains('|')
}

fn get_dir_type(path: &Path) -> PathType {
    let path = path.to_str().unwrap();
    let path: Vec<&str> = path.split("/").collect();

    match path.len() {
        1 => PathType::Artist,
        2 => PathType::Album,
        _ => PathType::Playlist,
    }
}

pub fn list_medialibrary(path: &Path,
    _ordering: FoldersOrdering) -> Result<AudioFolder, io::Error> {
    
    let files = vec![];
    let mut subfolders = vec![];
    let cover = None;
    let description = None;

    match get_dir_type(path) {
        PathType::Artist => { 
            subfolders = get_artists();
        },
        PathType::Album => {
            subfolders = get_artist("Muse");
        }
        _ => {},
    }

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
