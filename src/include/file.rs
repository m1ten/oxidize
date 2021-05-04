use std::fs::OpenOptions;
use std::io::{Read, Write};

// read from file
pub fn read_file(file_name: String) -> String {
    // open file
    let mut file = OpenOptions::new().read(true).open(&file_name).unwrap();

    // get contents from opened file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => println!("error reading from file: {}.", file_name),
    }

    // return contents
    contents
}

// write to file
pub fn write_file(file_name: String, text: String) {
    // open file with specific permissions
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_name)
        .unwrap();

    // convert text string to byte
    let text = text.as_bytes();

    // write to file
    match file.write_all(text) {
        Ok(_) => (),
        Err(_) => println!("error writing to file: {}.", file_name),
    }
}

pub fn set_path(path: &str) {
    let root = std::path::Path::new(path);

    if cfg!(debug_assertions) {
        dbg!(root.display());
    }

    // check if path is ok
    assert!(std::env::set_current_dir(&root).is_ok());
}