# list-files

A simple CLI tool built with Rust to list files in a specified directory, showing details such as permissions, size, and modification time.

## Features
- Lists files in the specified directory.
- Displays file permissions, size, and modification date.
- Supports viewing hidden files.
- Can specify a directory or use the current directory by default.

## Usage
cargo run /path/to/directory

## Example

```bash
$ cargo run             
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/list-files`
100644 99         2024-10-26 21:52:23  Cargo.toml
40755 192        2024-10-26 22:35:50  target
100644 8260       2024-10-26 20:15:26  Cargo.lock
100644 408        2024-10-26 22:44:08  README.md
100644 8          2024-10-26 19:16:00  .gitignore
40755 384        2024-10-26 22:35:26  .git
40755 96         2024-10-26 19:16:00  src
```