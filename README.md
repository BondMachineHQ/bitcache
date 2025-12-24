# bitcache

A command-line tool for managing binary files (bitstreams) in a git repository based on source file MD5 hashes.

## Overview

`bitcache` is a Rust-based tool designed to manage binary artifacts in a git repository by linking them to source files via MD5 hashes. It provides two main operations:

- **publish**: Computes the MD5 hash of a source file and uploads a binary file to a git repository at a specified path
- **get**: Retrieves a binary file from the repository based on the MD5 hash of its corresponding source file

The tool maintains a JSON metadata file (`bitcache_metadata.json`) in the repository to track all published binaries and their relationships to source files.

## Features

- 🔐 **MD5-based tracking**: Links binaries to source files using MD5 hashes
- 📦 **Git integration**: Uses git for version control and distribution
- 📝 **Metadata management**: Maintains a JSON file with complete tracking information
- 🚀 **Simple workflow**: Easy publish and retrieve operations
- ⏱️ **Timestamping**: Records publication time for each binary

## Prerequisites

- **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: Required for repository operations
- **Git authentication**: The tool uses git commands, so ensure you have appropriate access to the repository (SSH keys, tokens, etc.)

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/BondMachineHQ/bitcache.git
cd bitcache

# Build the project
cargo build --release

# The binary will be available at:
# target/release/bitcache
```

### Add to PATH (Optional)

```bash
# Copy to a directory in your PATH
sudo cp target/release/bitcache /usr/local/bin/

# Or add the target directory to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

## Usage

### Basic Syntax

```bash
bitcache <COMMAND> [OPTIONS]
```

### Commands

#### Publish

Upload a binary file to the repository with MD5-based tracking:

```bash
bitcache publish \
  --repo <REPOSITORY_URL> \
  --source <SOURCE_FILE> \
  --bitstream <BINARY_FILE> \
  --path <TARGET_DIRECTORY>
```

**Arguments:**
- `--repo`: Git repository URL (e.g., `https://github.com/user/repo.git`)
- `--source`: Path to the source file (used for MD5 computation)
- `--bitstream`: Path to the binary file to upload
- `--path`: Target directory path in the repository where the binary will be stored

**Example:**
```bash
bitcache publish \
  --repo https://github.com/myorg/bitstreams.git \
  --source design.vhd \
  --bitstream output.bit \
  --path builds/fpga
```

**What happens:**
1. Computes MD5 hash of `design.vhd`
2. Clones the repository to a temporary location
3. Copies `output.bit` to `builds/fpga/` in the repository
4. Updates `bitcache_metadata.json` with the new entry
5. Commits and pushes changes to the repository

#### Get

Retrieve a binary file from the repository by its source MD5 hash:

```bash
bitcache get \
  --repo <REPOSITORY_URL> \
  --md5 <MD5_HASH>
```

**Arguments:**
- `--repo`: Git repository URL
- `--md5`: MD5 hash of the source file

**Example:**
```bash
bitcache get \
  --repo https://github.com/myorg/bitstreams.git \
  --md5 a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
```

**What happens:**
1. Clones the repository to a temporary location
2. Reads `bitcache_metadata.json`
3. Finds the binary associated with the given MD5
4. Copies the binary to the current directory

## Metadata Format

The tool maintains a `bitcache_metadata.json` file in the root of the repository with the following structure:

```json
{
  "entries": {
    "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6": {
      "md5": "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
      "binary_path": "builds/fpga/output.bit",
      "source_file": "design.vhd",
      "timestamp": "2025-12-11T10:30:00Z"
    }
  }
}
```

**Fields:**
- `md5`: MD5 hash of the source file
- `binary_path`: Relative path to the binary file in the repository
- `source_file`: Original source filename
- `timestamp`: ISO 8601 timestamp of when the binary was published

## Examples

### Example 1: Publish a Bitstream

```bash
# Publish an FPGA bitstream linked to a VHDL source file
bitcache publish \
  --repo git@github.com:myorg/fpga-builds.git \
  --source src/top.vhd \
  --bitstream build/top.bit \
  --path releases/v1.0
```

Output:
```
Publishing bitstream...
Computing MD5 of source file: src/top.vhd
MD5: 3f4d8a9b2c1e5f7a8b9c0d1e2f3a4b5c
Cloning repository: git@github.com:myorg/fpga-builds.git
Copying bitstream to: releases/v1.0/top.bit
Updating metadata...
Committing and pushing changes...
Successfully published bitstream with MD5: 3f4d8a9b2c1e5f7a8b9c0d1e2f3a4b5c
```

### Example 2: Retrieve a Bitstream

```bash
# Retrieve the bitstream for a specific source MD5
bitcache get \
  --repo git@github.com:myorg/fpga-builds.git \
  --md5 3f4d8a9b2c1e5f7a8b9c0d1e2f3a4b5c
```

Output:
```
Retrieving bitstream for MD5: 3f4d8a9b2c1e5f7a8b9c0d1e2f3a4b5c
Cloning repository: git@github.com:myorg/fpga-builds.git
Copying top.bit to /current/directory/top.bit
Successfully retrieved bitstream:
  Source file: top.vhd
  MD5: 3f4d8a9b2c1e5f7a8b9c0d1e2f3a4b5c
  Timestamp: 2025-12-11T10:30:00Z
  Saved to: /current/directory/top.bit
```

### Example 3: Workflow Integration

```bash
#!/bin/bash
# Build script that publishes bitstreams

SOURCE_FILE="design.v"
BITSTREAM="build/design.bit"
REPO="git@github.com:myorg/bitstreams.git"

# Build the design
make clean
make bitstream

# Compute MD5 to check if we need to publish
MD5=$(md5sum $SOURCE_FILE | awk '{print $1}')
echo "Build completed. Source MD5: $MD5"

# Publish to repository
bitcache publish \
  --repo $REPO \
  --source $SOURCE_FILE \
  --bitstream $BITSTREAM \
  --path builds/$(date +%Y-%m-%d)

echo "Bitstream published. To retrieve later, use:"
echo "bitcache get --repo $REPO --md5 $MD5"
```

## How It Works

### Publish Workflow

1. **MD5 Computation**: Computes the MD5 hash of the source file
2. **Repository Clone**: Clones the git repository to a temporary directory
3. **Metadata Loading**: Loads existing metadata or creates new file
4. **File Copy**: Copies the binary file to the specified path in the repository
5. **Metadata Update**: Adds or updates the entry for the MD5 hash
6. **Git Operations**: Commits and pushes changes back to the repository
7. **Cleanup**: Temporary directory is automatically cleaned up

### Get Workflow

1. **Repository Clone**: Clones the git repository to a temporary directory
2. **Metadata Lookup**: Reads the metadata file and searches for the MD5
3. **File Retrieval**: Locates the binary file in the repository
4. **File Copy**: Copies the binary to the current working directory
5. **Cleanup**: Temporary directory is automatically cleaned up

## Use Cases

- **FPGA Development**: Track bitstreams generated from HDL source files
- **Compiler Output**: Cache compiled binaries linked to source code versions
- **Build Artifacts**: Manage binary outputs from complex build processes
- **Reproducible Builds**: Ensure binaries can be retrieved for specific source versions
- **Team Collaboration**: Share binary artifacts across team members

## Troubleshooting

### Common Issues

1. **Git authentication fails**
   - Ensure you have proper SSH keys or credentials configured
   - Test with `git clone <repo>` manually
   - Use HTTPS URLs with tokens if SSH is not available

2. **Permission denied on push**
   - Verify you have write access to the repository
   - Check that the repository exists and is accessible

3. **MD5 not found**
   - Ensure the MD5 hash is correct
   - The binary must have been published first
   - Check the repository's `bitcache_metadata.json` file

4. **File already exists**
   - When using `get`, if the file exists in current directory, remove it first
   - Or rename the existing file before retrieving

## Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check for common mistakes
cargo clippy
```

### Project Structure

```
bitcache/
├── Cargo.toml          # Project dependencies and metadata
├── LICENSE             # License information
├── README.md           # This file
└── src/
    └── main.rs         # Main source code with all functionality
```

## Inspiration

This tool was inspired by [bmregression](https://github.com/BondMachineHQ/bmregression), a regression testing tool for the BondMachine project.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

See [LICENSE](LICENSE) file for details.

## Related Projects

- [bmregression](https://github.com/BondMachineHQ/bmregression) - Regression testing tool for BondMachine
- [BondMachine](https://github.com/BondMachineHQ/BondMachine) - Main BondMachine project

Tool to fetch and publish BM pre-build bitstreams
