use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;

fn main() {
    list_files();
}

fn list_files() {
    // カレントディレクトリを取得
    let current_dir = env::current_dir().expect("Could not get current directory");

    let entries = fs::read_dir(&current_dir).expect("Could not read directory");

    for entry in entries {
        let entry = entry.expect("Error reading etnry");
        let metadata = entry.metadata().expect("Could not read metadata");

        // パーミッション取得
        let permissions = metadata.permissions().mode();
        // ファイルサイズ取得
        let file_size = metadata.len();
        // ファイルの最終更新日時
        let modified = metadata.modified().expect("Could not get modified time");
        let datetime: chrono::DateTime<chrono::Local> = modified.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();


        if let Some(file_name) = entry.file_name().to_str() {
            println!(
                "{:o} {:<10} {:<20} {}", 
                permissions,
                file_size,
                formatted_time,
                file_name
            );
        }
    }
}