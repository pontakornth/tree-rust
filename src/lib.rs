use std::fs;
use colored::*;

pub fn repeat_char(ch: char, n: i32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push(ch);
    }
    result
}



// TODO: Refactor into some kind of print option type
pub fn print_dir(path: &str, padding: i32, current_depth: i32, max_depth: i32) {
    let inner_paths = fs::read_dir(path).unwrap();
    // When beyond max_depth return 
    // Box drawing character ─
    if max_depth != -1 && current_depth - 1 == max_depth {
        return
    }
    for path in inner_paths {
        let entry = path.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            let file_path = entry.path();
            println!("├{} {}", repeat_char(' ', padding), file_name.green());
            print_dir(file_path.to_str().unwrap(), padding + 2, current_depth +1, max_depth);
        } else {
            println!("├{} {}", repeat_char(' ', padding), file_name.blue());
        }
    }
}
