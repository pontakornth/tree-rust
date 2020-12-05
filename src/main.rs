use std::fs;

fn repeat_char(ch: char, n: i32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push(ch);
    }
    result
}

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{} {}", repeat_char('-', 3), path.unwrap().file_name().into_string().unwrap());
    }
    println!("Hello, world!");
}
