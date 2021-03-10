use std::path::{Path};

use crate::services::types::{AudioFolderShort};

const ARTISTS: &str   = "artists";
const ALBUMS: &str    = "albums";
const TRACKS: &str    = "tracks";
const PLAYLISTS: &str = "playlists";

pub enum PathType {
    Artist,
    Album,
    Track,
    Playlist,
    Other
}

pub struct LibPath {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track: Option<String>,
    pub entity: PathType,
}

impl LibPath {
    pub fn from_path(path: &Path) -> Self {
        let path = path.to_str().unwrap();
        let path: Vec<&str> = path[1..].split("/").collect();
        //let entity = get_dir_type(&path);
        let mut lib_path = LibPath { 
            artist: None, album: None, track: None, entity: PathType::Other };
        
        match get_root_type(path[0]) {
            PathType::Artist => match path.len() {
                1  => { lib_path.entity = PathType::Artist },
                2  => { 
                    lib_path.artist = Some(String::from(path[1]));
                    lib_path.entity = PathType::Artist; 
                },
                3  => { 
                    lib_path.artist = Some(String::from(path[1]));
                    lib_path.album = Some(String::from(path[2]));
                    lib_path.entity = PathType::Album; 
                },
                4  => { 
                    lib_path.artist = Some(String::from(path[1]));
                    lib_path.album = Some(String::from(path[2]));
                    lib_path.track = Some(String::from(path[3]));
                    lib_path.entity = PathType::Track; 
                },
                _  => {},
            },
            PathType::Album => match path.len() {
                1 => { lib_path.entity = PathType::Album },
                2 => { 
                    lib_path.album = Some(String::from(path[1]));
                    lib_path.entity = PathType::Album; 
                },
                3 => { 
                    lib_path.album = Some(String::from(path[1]));
                    lib_path.track = Some(String::from(path[2]));
                    lib_path.entity = PathType::Track; 
                },
                _ => {},
            },
            PathType::Track => match path.len() {
                1 => { lib_path.entity = PathType::Track },
                2 => { 
                    lib_path.track = Some(String::from(path[1]));
                    lib_path.entity = PathType::Track; 
                },
                _ => {},
            },
            _                => {},
        };

        lib_path

        /*
        LibPath { 
            artist: String::from(""),
            album: String::from("Showbiz"),
            track: String::from("Uno.mp3"),
            entity: entity,
        }
        */
    }
}

pub fn is_lib_path(path: &Path) -> bool {
    path.to_str().unwrap().contains('|')
}

fn get_root_type(part: &str) -> PathType {
    match part {
        ARTISTS => PathType::Artist,
        ALBUMS  => PathType::Album,
        TRACKS  => PathType::Track,
        _       => PathType::Other,
    }
}

fn get_dir_type(path: &Vec<&str>) -> PathType {
    match get_root_type(path[0]) {
        PathType::Artist => match path.len() {
            1 | 2 => PathType::Artist,
            3     => PathType::Album,
            4     => PathType::Track,
            _     => PathType::Other,
        },
        PathType::Album  => match path.len() {
            1 | 2 => PathType::Album,
            3     => PathType::Track,
            _     => PathType::Other,
        },
        PathType::Track  => PathType::Track,
        _                => PathType::Other,
    }
}

pub fn insert_mediadb(subfolders: &mut Vec<AudioFolderShort>) {
    subfolders.insert(0, AudioFolderShort::from_mdb_root(ARTISTS));
    subfolders.insert(1, AudioFolderShort::from_mdb_root(ALBUMS));
    subfolders.insert(2, AudioFolderShort::from_mdb_root(TRACKS));
    //subfolders.insert(3, AudioFolderShort::from_mdb_root(PLAYLISTS));
}
