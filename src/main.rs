use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    // 引数を取得
    let args: Vec<String> = env::args().collect();

    // もし引数が指定されていなければカレントディレクトリを使用
    let dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    list_files(dir);
}

fn list_files(dir:  &str) {
    let path = Path::new(dir);

    let entries = fs::read_dir(path).expect("Could not read directory");

    // ヘッダーを表示
    println!("{:<10} {:<10} {:<20} {}", "Permissions", "Size", "Modified", "Name");

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