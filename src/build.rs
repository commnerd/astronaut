extern crate glob;

use glob::{glob, Paths};

use std::fs::File;
use std::io::{BufWriter, Write};

fn write_mod(framework_dir: &'static str, dir: String, path: String, paths: Paths) -> std::io::Result<()> {
    let file = File::create(&path)?;
    let mut bw = BufWriter::new(file);
    let mut mods = Vec::<String>::new();
    let mut mod_contents = String::from("");
    for mod_path_buf in paths {
        let mod_path = mod_path_buf.as_ref().expect("Nope").display().to_string();
        let module = String::from(&mod_path.clone()[String::from(&dir).len() + 1..mod_path.clone().len() - 3]);
        match mod_path_buf {
            Ok(confirmed_path_buf) => {
                let confirmed_path = confirmed_path_buf.display().to_string();
                if confirmed_path[&confirmed_path.len() - 6..].to_string() != String::from("mod.rs") {
                    mods.push(String::from(module));
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
    
    mod_contents.push_str("// DO NOT EDIT!  THIS FILE IS AUTOMATICALLY GENERATED BY ASTRONAUT!\n\n");

    for module in &mods {
        mod_contents.push_str("pub mod ");
        mod_contents.push_str(&module);
        mod_contents.push_str(";\n");
    }

    match framework_dir.to_string().as_str() {
        "config" => {
            mod_contents.push_str("\n");
            mod_contents.push_str("pub fn read() -> Value {\n");
            mod_contents.push_str(" astronaut::config::new()");
            for module in &mods {
                mod_contents.push_str(format!("\n     .add({}.read())", module).as_str());
            }
            mod_contents.push_str(";\n");
            mod_contents.push_str("}\n");
        },
        _ => ()
    }

    bw.write_all(mod_contents.as_bytes()).expect("Unable to write data");
    Ok(())
}

pub fn build() {
    let basedir = std::env::var("CARGO_MANIFEST_DIR").expect("Can't find project directory.");
    for dir in vec!["config"] {
        let glob_dir = format!("{}/src/{}", basedir, dir);
        let mod_path = format!("{}/mod.rs", glob_dir);
        let glob_path = format!("{}/*.rs", glob_dir);
        let dir_list_failure = format!("Failed to read contents of {} directory.", dir);
        let write_failure = format!("Something went wrong writing mod.rs in {}.", dir);

        write_mod(dir, String::from(glob_dir), mod_path, glob(&glob_path).expect(&dir_list_failure)).expect(&write_failure);
    }
}