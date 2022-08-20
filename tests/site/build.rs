extern crate glob;
use glob::glob;

fn main() {
    for e in glob("./config/*").expect("Failed to read glob pattern") {
        println!("{}", e.unwrap().display());
    }
}