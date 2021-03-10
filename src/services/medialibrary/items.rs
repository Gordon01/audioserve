use std::io;
use std::collections::HashMap;
use std::{sync::Mutex};
use std::path::{Path, PathBuf};

use super::types::*;
use super::paths::{PathType, LibPath};
use crate::services::types::*;

lazy_static! {
    static ref ARTISTS: Mutex<HashMap<String, Artist>> = {
        let mut m: HashMap<String, Artist> = HashMap::new();
        m.insert("Muse".to_string(), Artist {albums: 1, tracks: 1});
        m.insert("Radiohead".to_string(), Artist {albums: 1, tracks: 1});
        Mutex::new(m)
    };
}

pub fn get_item(path: &Path,
    _ordering: FoldersOrdering) -> Result<AudioFolder, io::Error> {
    
    let files = vec![];
    let cover = None;
    let description = None;

    let lib_path = LibPath::from_path(path);

    let subfolders = match lib_path.entity {
        PathType::Artist => get_artist(&lib_path),
        PathType::Album  => get_album(&lib_path),
        PathType::Track  => get_track(&lib_path),
        _ => {vec![]},
    };

    Ok(AudioFolder {
        files,
        subfolders,
        cover,
        description,
    })
}

fn get_artist(path: &LibPath) -> Vec<AudioFolderShort> {
    let mut artist = vec![];
    match path.artist.as_ref().unwrap().as_str() {
        "Muse" => {
            artist.push(AudioFolderShort::from_mdb_root("Showbiz"));
            artist.push(AudioFolderShort::from_mdb_root("Absolution"));
        },
        "Radiohead" => {
            artist.push(AudioFolderShort::from_mdb_root("OK Computer"));
            artist.push(AudioFolderShort::from_mdb_root("In Rainbows"));
        },
        _ => { artist = get_artists() },
    };
    artist
}

fn get_album(path: &LibPath) -> Vec<AudioFolderShort> {
    let mut artist = vec![];
    
    artist.push(AudioFolderShort::from_mdb_root("Showbiz"));
    artist.push(AudioFolderShort::from_mdb_root("Absolution"));

    artist
}

fn get_track(path: &LibPath) -> Vec<AudioFolderShort> {
    let mut artist = vec![];

    artist.push(AudioFolderShort::from_mdb_root("Uno.mp3"));
    artist.push(AudioFolderShort::from_mdb_root("Muscle Museum.mp3"));

    artist
}

fn get_artists() -> Vec<AudioFolderShort> {
    let artists = ARTISTS.lock().unwrap();
    let as_folders = artists.iter().map(
        |(x, y)| 
        AudioFolderShort::from_path_and_name(x.to_string(), PathBuf::from("|artists/Muse"), false));
    as_folders.collect()
}
