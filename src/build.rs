extern crate glob;

use glob::{glob, Paths};

use std::fs::File;
use std::io::{BufWriter, Write};


fn test() -> std::io::Result<()> {
    let file = File::create("/tmp/test")?;
    let mut bw = BufWriter::new(file);
    bw.write_all("Goodbye".as_bytes()).expect("Unable to write data");
    Ok(())
}
fn write_mod(dir: String, path: String, paths: Paths) -> std::io::Result<()> {
    let file = File::create(&path)?;
    let mut bw = BufWriter::new(file);
    let mut mods = Vec::<&str>::new();
    let mut mod_contents = "".to_owned();
    let path_string = path.to_string();
    for entry in paths {
        let module = &path_string[dir.len() + 5..path_string.len() - 3];
        match entry {
            Ok(_) => {
                if path_string[path_string.len() - 6..] != String::from("mod.rs") {
                    mods.push(&module);
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
    test();

    for module in &mods {
        mod_contents.push_str("pub mod ");
        mod_contents.push_str(module);
        mod_contents.push_str(";\n");

        match module {
            &"config" => {
                mod_contents.push_str("\n");
                mod_contents.push_str("pub fn read() -> Value {\n");
                mod_contents.push_str(" astronaut::config::new()\n");
                for module in &mods {
                    mod_contents.push_str(format!("     .add({}.get())", module).as_str());
                }
                mod_contents.push_str(";\n");
                mod_contents.push_str("}")
            }
            _ => ()
        }
    }

    bw.write_all(mod_contents.as_bytes()).expect("Unable to write data");
    Ok(())
}

pub fn build() {
    for dir in vec!["config"] {
        let mod_path = format!("src/{}/mod.rs", dir);
        let glob_path = format!("src/{}/*.rs", dir);
        let dir_list_failure = format!("Failed to read contents of {} directory.", dir);
        let write_failure = format!("Something went wrong writing mod.rs in {}.", dir);

        write_mod(String::from(dir), mod_path, glob(&glob_path).expect(&dir_list_failure)).expect(&write_failure);
    }
}