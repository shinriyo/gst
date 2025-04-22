# gst (Git Skip-worktree Tool)

A Terminal User Interface (TUI) tool for managing Git's skip-worktree flags, inspired by tig.

## Features

- Interactive TUI for managing skip-worktree flags
- Easy navigation and toggle functionality
- Bulk operations support
- Command-line interface for quick operations

## Installation

### From Source

```bash
cargo install --path .
```

### Using Homebrew

```bash
brew tap shinriyo/gst
brew install gst
```

## Usage

### TUI Mode

Simply run `gst` in your Git repository:

```bash
gst
```

#### Key Bindings

- `j` / `↓`: Move cursor down
- `k` / `↑`: Move cursor up
- `u`: Toggle skip-worktree flag for selected file
- `!`: Clear all skip-worktree flags (with confirmation)
- `q`: Quit

### CLI Mode

Skip files:
```bash
gst skip file1 file2
```

Resume tracking:
```bash
gst resume file1 file2
```

## Why use skip-worktree?

The skip-worktree flag is useful when you want Git to ignore local changes to tracked files. Common use cases include:

- Configuration files with local modifications
- Environment-specific settings
- Local development tweaks

Unlike `.gitignore`, skip-worktree works with files that are already tracked by Git.

## Requirements

- Git
- Rust 2021 edition or later

## License

MIT

## Author

shinriyo
