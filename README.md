# list-files

A simple CLI tool built with Rust to list files in a specified directory, showing details such as permissions, size, and modification time.

## Features
- Lists files in the specified directory.
- Displays file permissions, size, and modification date.
- Supports viewing hidden files.
- Can specify a directory or use the current directory by default.

## Installation
```
curl -sL https://github.com/yourusername/yourrepository/releases/download/v0.1.0/lf -o /usr/local/bin/lf
chmod +x /usr/local/bin/lf
```

## Usage
`lf [directory]`

If [directory] is not specified, the current directory will be used.
The output includes file permissions, size (with units), last modified date, and file name.

## Example

```bash
Permissions Size       Modified             Name
-rw-r--r--  140 B      2024-10-28 14:59:42  Cargo.toml
drwxr-xr-x  192 B      2024-10-28 14:53:31  target
-rw-r--r--  8.07 KB    2024-10-26 20:15:26  Cargo.lock
-rw-r--r--  895 B      2024-10-27 00:21:03  README.md
-rw-r--r--  8 B        2024-10-26 19:16:00  .gitignore
drwxr-xr-x  384 B      2024-10-28 15:29:35  .git
-rwxr-xr-x  517.48 KB  2024-10-28 14:54:45  lf
drwxr-xr-x  96 B       2024-10-26 19:16:00  src
```