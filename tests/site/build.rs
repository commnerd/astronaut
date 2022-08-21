extern crate glob;

use glob::{glob, Paths};

use std::fs::File;
use std::io::{BufWriter, Write};

fn write_mod(dir: String, path: String, paths: Paths) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut bw = BufWriter::new(file);
    let mut mods = "".to_owned();
    for entry in paths {
        match entry {
            Ok(path) => {
                let path_string = path.display().to_string();
                if path_string[path_string.len() - 6..] != String::from("mod.rs") {
                    let path = path.display().to_string();
                    mods.push_str("pub mod ");
                    mods.push_str(&path[dir.len() + 5..path.len() - 3]);
                    mods.push_str(";\n");
                }
            },
            Err(e) => println!("{:?}", e),
        }
        
    }
    bw.write_all(mods.as_bytes()).expect("Unable to write data");
    Ok(())
}

fn main() {
    for dir in vec!["config"] {
        let mod_path = format!("src/{}/mod.rs", dir);
        let glob_path = format!("src/{}/*.rs", dir);
        let dir_list_failure = format!("Failed to read contents of {} directory.", dir);
        let write_failure = format!("Something went wrong writing mod.rs in {}.", dir);

        write_mod(String::from(dir), mod_path, glob(&glob_path).expect(&dir_list_failure)).expect(&write_failure);
    }
}