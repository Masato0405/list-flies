use std::fs;
use std::env;

fn main() {
    // カレントディレクトリを取得
    let current_dir = env::current_dir().expect("Could not get current directory");

    list_files(&current_dir);
}

fn list_files(dir: &std::path::Path) {
    let entries = fs::read_dir(dir).expect("Could not read directory");

    for entry in entries {
        let entry = entry.expect("Error reading etnry");

        if let Some(file_name) = entry.file_name().to_str() {
            println!("{}", file_name);
        }
    }
}