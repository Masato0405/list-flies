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
        let permissions = format_permissions(&metadata);
        // ファイルサイズ取得
        let file_size = format_file_size(metadata.len());
        // ファイルの最終更新日時
        let modified = metadata.modified().expect("Could not get modified time");
        let datetime: chrono::DateTime<chrono::Local> = modified.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();


        if let Some(file_name) = entry.file_name().to_str() {
            println!(
                "{:11} {:<10} {:<20} {}", 
                permissions,
                file_size,
                formatted_time,
                file_name
            );
        }
    }
}

fn format_permissions(metadata: &std::fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    let owner = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' }
    );

    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' }
    );

    let others = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' }
    );

    format!("{}{}{}{}", file_type, owner, group, others)
}

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size as f64 / GB as f64)
    }
}