use std::collections::HashMap;
use std::{sync::Mutex};
use std::path::{Path, PathBuf};

use super::types::*;
use crate::services::types::*;

lazy_static! {
    static ref ARTISTS: Mutex<HashMap<String, Artist>> = {
        let mut m: HashMap<String, Artist> = HashMap::new();
        m.insert("Muse".to_string(), Artist {albums: 1, tracks: 1});
        Mutex::new(m)
    };    
}

pub fn get_artists() -> Vec<AudioFolderShort> {
    let artists = ARTISTS.lock().unwrap();
    let as_folders = artists.iter().map(
        |(x, y)| 
        AudioFolderShort::from_path_and_name(x.to_string(), PathBuf::from("|artists/Muse"), false));
    as_folders.collect()
}

pub fn get_artist(name: &str) -> Vec<AudioFolderShort> {
    let mut artist = vec![];
    if name == "Muse" {
        artist.push(AudioFolderShort::from_label("Showbiz"));
        artist.push(AudioFolderShort::from_label("Absolution"));
    }
    artist.push(AudioFolderShort::from_label("Tracks"));
    artist
}