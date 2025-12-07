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
    fn scan(&self, path: String, file_name: bool) -> Vec<String> {

        let paths = fs::read_dir(path).unwrap();

        let mut list: Vec<String> = Vec::new();

        for path in paths {
            
            match file_name {
                false => {
                    list.push(path.unwrap()
                        .path()
                        .display()
                        .to_string());
                },
                true  => {
                    list.push(path.unwrap()
                        .path()
                        .as_path()
                        .file_name()
                        .expect("FAILED")
                        .display()
                        .to_string());
                },
                _ => println!("FF"),
            }
        }
        list
    }

    //Filtre pk3
    pub fn list_pk3(&self, file_name: bool) -> Vec<String> {
        let list: Vec<String> = self.scan(self.config.directories.dir_pk3.clone(), file_name);
        let mut list_pk3: Vec<String> = Vec::new();

        let ext_pk3s: Vec<&str> = vec![".pk3", ".ipk3", ".Pk3"];

        for i in list {
            for j in &ext_pk3s {
                if i.contains(&*j) {
                    list_pk3.push(i.clone());
                }
            }
        }

        list_pk3
    }

    //Filtre WADs
    pub fn list_wad(&self, file_name: bool) -> Vec<String> {
        let list: Vec<String> = self.scan(self.config.directories.dir_wads.clone(), file_name);
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

