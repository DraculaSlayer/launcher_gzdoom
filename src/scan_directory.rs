use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use std::path::{self, Path};
use std::env;

#[derive(Debug, Deserialize)]
struct Config {
    directories: Directories,
}

#[derive(Debug, Deserialize)]
struct Directories {
    dir_wads: String,
    dir_pk3:  String,
}

pub struct ScanDir {
    config: Config,
}

impl ScanDir {
    pub fn new() -> Self {

        let home = env::home_dir().expect("FAILED").display().to_string();

        let mut file = File::open(format!("{}/.config/launcher_gzdoom/config.toml", home)).unwrap();

        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        Self {
            config: toml::from_str(&content.as_str()).unwrap(),
        }
    }

    //Scan the directory
    fn scan(&self, path: String) -> Vec<String> {

        let paths = fs::read_dir(path).unwrap();

        let mut list: Vec<String> = Vec::new();

        for path in paths {

            let element = path.unwrap()
                .path()
                .as_path()
                .file_name()
                .expect("FAILED")
                .display()
                .to_string();

            list.push(element);
        }
        list
    }

    //Filtre pk3
    pub fn list_pk3(&self) -> Vec<String> {
        let list: Vec<String> = self.scan(self.config.directories.dir_pk3.clone());
        let mut list_pk3: Vec<String> = Vec::new();

        for i in list {
            if i.contains(".pk3") {
                list_pk3.push(i);
            }
        }

        list_pk3
    }

    //Filtre WADs
    pub fn list_wad(&self) -> Vec<String> {
        let list: Vec<String> = self.scan(self.config.directories.dir_wads.clone());
        let mut list_wad: Vec<String> = Vec::new();

        let ext_wads: Vec<&str> = vec![".wad", ".iwad", ".WAD"];

        for i in list {
            for j in &ext_wads {
                if i.contains(&*j) {
                    list_wad.push(i.clone());
                }
            }
        }

        list_wad
    }
}

