# Resources Builder for Rusttale

This tool is a helper utility designed to automate the management and packaging of game assets for **Rusttale**.

It fetches raw resources (images, audio, etc.) from a specified source (usually a remote Git repository), processes them, and serializes them into optimized binary archives (e.g., `sounds.bin`, `images.bin`) ready to be loaded by the game engine.

## Features

- **Automated Retrieval:** Clones the latest assets from a remote Git repository into a temporary environment.
- **Asset Discovery:** Recursively scans for specific asset types:
  - **Audio:** `.mp3`, `.wav` (in `music/`, `sounds/` directories)
  - **Images:** `.png`, `.jpg` (in `png/`, `background/` directories)
- **Binary Serialization:** Compiles discovered assets into single binary files for efficient loading and distribution.
- **Safe Execution:** Uses temporary directories for processing to keep your workspace clean.

## Usage

You can run this tool directly using Cargo.

### Arguments

- `--source <URL>`: The URL of the Git repository containing the raw resources.
  - Default: `https://github.com/lazypwny751/resourcetale`
- `--destdir <PATH>`: The destination directory where the generated binary files (`.bin`) will be saved.
  - Default: `bin`

### Examples

**Run with default settings:**
```bash
cargo run
```

**Specify a custom source repository and destination:**
```bash
cargo run -- --source https://github.com/my-user/my-assets-repo --destdir ./assets/packed
```

## Building

To build the release version of the tool:

```bash
cargo build --release
```
