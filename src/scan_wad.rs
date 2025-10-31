use std::fs;

pub fn list_wad() -> Vec<String> {
    let paths = fs::read_dir("./").unwrap();
    let mut list_wads: Vec<String> = Vec::new();

    for path in paths {

        let element = path.unwrap().path().display().to_string();

        list_wads.push(element);
    }
    list_wads
}
