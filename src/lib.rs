use std::fs;
use colored::*;

pub fn repeat_char(ch: char, n: i32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push(ch);
    }
    result
}


pub fn print_dir(path: &str, padding: i32) {
    let inner_paths = fs::read_dir(path).unwrap();
    for path in inner_paths {
        let entry = path.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            let file_path = entry.path();
            println!("├{} {}", repeat_char('─', padding), file_name.green());
            print_dir(file_path.to_str().unwrap(), padding + 4);
        } 
        println!("├{} {}", repeat_char('─', padding), file_name.blue());
    }
}
