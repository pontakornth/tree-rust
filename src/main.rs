use std::fs;
fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
    println!("Hello, world!");
}
