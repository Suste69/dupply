# Dupply

A fast, lightweight duplicate file finder built in Rust.

Dupply scans directories recursively, identifies potential duplicate files using SHA-256 hashing, and reports how much disk space could be recovered.

## Features

- Recursive directory scanning
- File size filtering
- SHA-256 duplicate detection
- Memory-efficient file hashing
- Duplicate statistics
- Read-only operation: no automatic file deletion

## Installation

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/dupply.git
cd dupply
cargo build --release
```

## Usage

Scan a directory:

```bash
cargo run -- scan ./test_files
```

Scan with a minimum file size:

```bash
cargo run -- scan ~/Downloads --min-size 1024
```

Run the compiled executable:

```bash
./target/release/dupply scan ~/Downloads
```

## How It Works

1. Recursively scan a directory.
2. Group files by their sizes.
3. Ignore groups containing only one file.
4. Calculate SHA-256 hashes using buffered reads.
5. Group files with identical hashes.
6. Report duplicate groups and potentially recoverable disk space.

## Testing

```bash
cargo test
cargo clippy --all-targets
cargo fmt -- --check
```

## Limitations

- SHA-256 matches are not additionally verified byte by byte.
- Files modified during scanning may produce inconsistent results.
- Filesystem access errors can interrupt scanning.
- Dupply does not automatically delete duplicate files.
