# Toolbox

A modern, modular Rust CLI tool that bundles several developer utilities into a single binary.

## Features

- **Organize**: Automatically sort files in a directory by type (extension) or modification date.
- **Git Helper**: Summarize repo status and clean up merged branches.
- **Todo List**: Persistent todo manager stored in your config directory.
- **Bulk Rename**: Rename files in bulk using regex patterns.
- **Dir Size**: Find the largest files in a directory tree.

## Usage

Build the tool:
```sh
cargo build --release
```

Run a command:
```sh
# Organize files by type
cargo run -- organize --mode type

# Git summary
cargo run -- git-helper --summary

# Add a todo
cargo run -- todo add "Try out my Rust CLI!"

# Bulk rename files
cargo run -- bulk-rename --pattern "foo" --replace "bar"

# Directory size analysis
cargo run -- dir-size --top 5
```

## Requirements
- Rust (latest stable, via rustup recommended)
- For git-helper: a git repository in the target directory

## Contributing
Pull requests and issues are welcome! Please open an issue to discuss major changes.

## License
MIT
