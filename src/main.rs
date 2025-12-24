//! # bitcache
//!
//! A tool for managing binary files (bitstreams) in a git repository based on source file MD5 hashes.
//!
//! ## Overview
//!
//! This tool provides two main operations:
//! - **publish**: Computes MD5 of a source file and uploads a binary file to a git repository
//! - **get**: Retrieves a binary file from the repository based on its MD5 hash
//!
//! ## Workflow
//!
//! 1. The tool maintains a JSON metadata file in the repository
//! 2. When publishing, it computes the MD5 hash of the source file
//! 3. The binary file is stored at the specified path with metadata tracking
//! 4. When getting, it looks up the MD5 in metadata and copies the binary to current directory

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Metadata entry for a cached binary file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetadataEntry {
    /// MD5 hash of the source file
    md5: String,
    /// Path to the binary file in the repository
    binary_path: String,
    /// Original source filename
    source_file: String,
    /// Timestamp of publication
    timestamp: String,
}

/// Root metadata structure
#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    /// Map of MD5 hash to metadata entry
    entries: HashMap<String, MetadataEntry>,
}

impl Metadata {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn load_from_file(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse metadata: {}", e),
            )
        })
    }

    fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(path, content)
    }
}

/// Command-line interface for bitcache
#[derive(Parser)]
#[command(version, about = "Binary file cache manager using git and MD5 hashing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Publish a binary file to the repository
    Publish {
        /// Git repository URL
        #[arg(long)]
        repo: String,

        /// Source file path
        #[arg(long)]
        source: PathBuf,

        /// Binary file (bitstream) path
        #[arg(long)]
        bitstream: PathBuf,

        /// Target directory path in the repository
        #[arg(long)]
        path: PathBuf,
    },
    /// Get a binary file from the repository by MD5
    Get {
        /// Git repository URL
        #[arg(long)]
        repo: String,

        /// MD5 hash of the source file
        #[arg(long)]
        md5: String,
    },
}

/// Compute MD5 hash of a file
fn compute_md5(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

/// Clone a git repository to a temporary location
fn clone_repository(repo_url: &str, target_dir: &Path) -> io::Result<()> {
    let output = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(target_dir)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to clone repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    Ok(())
}

/// Add, commit and push changes to the repository
fn commit_and_push(repo_dir: &Path, message: &str) -> io::Result<()> {
    // Add all changes
    let add_output = Command::new("git")
        .current_dir(repo_dir)
        .arg("add")
        .arg(".")
        .output()?;

    if !add_output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to add files: {}",
                String::from_utf8_lossy(&add_output.stderr)
            ),
        ));
    }

    // Commit changes
    let commit_output = Command::new("git")
        .current_dir(repo_dir)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()?;

    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        // Check if there's nothing to commit
        if stderr.contains("nothing to commit") {
            println!("No changes to commit");
            return Ok(());
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to commit: {}", stderr),
        ));
    }

    // Push changes
    let push_output = Command::new("git")
        .current_dir(repo_dir)
        .arg("push")
        .output()?;

    if !push_output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to push: {}",
                String::from_utf8_lossy(&push_output.stderr)
            ),
        ));
    }

    Ok(())
}

/// Handle the publish subcommand
fn handle_publish(
    repo: &str,
    source: &Path,
    bitstream: &Path,
    target_path: &Path,
) -> io::Result<()> {
    println!("Publishing bitstream...");

    // Compute MD5 of source file
    println!("Computing MD5 of source file: {}", source.display());
    let md5_hash = compute_md5(source)?;
    println!("MD5: {}", md5_hash);

    // Create temporary directory for repository
    let temp_dir = tempfile::tempdir()?;
    let repo_dir = temp_dir.path().join("repo");

    // Clone repository
    println!("Cloning repository: {}", repo);
    clone_repository(repo, &repo_dir)?;

    // Load or create metadata
    let metadata_path = repo_dir.join("bitcache_metadata.json");
    let mut metadata = if metadata_path.exists() {
        Metadata::load_from_file(&metadata_path)?
    } else {
        Metadata::new()
    };

    // Create target directory in repository
    let full_target_path = repo_dir.join(target_path);
    fs::create_dir_all(&full_target_path)?;

    // Get bitstream filename
    let bitstream_filename = bitstream
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid bitstream path"))?;

    // Copy bitstream to target location
    let dest_bitstream = full_target_path.join(bitstream_filename);
    println!(
        "Copying bitstream to: {}",
        dest_bitstream.strip_prefix(&repo_dir).unwrap().display()
    );
    fs::copy(bitstream, &dest_bitstream)?;

    // Update metadata
    let source_filename = source
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let binary_rel_path = dest_bitstream
        .strip_prefix(&repo_dir)
        .unwrap()
        .to_string_lossy()
        .to_string();

    let entry = MetadataEntry {
        md5: md5_hash.clone(),
        binary_path: binary_rel_path,
        source_file: source_filename,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    metadata.entries.insert(md5_hash.clone(), entry);

    // Save metadata
    println!("Updating metadata...");
    metadata.save_to_file(&metadata_path)?;

    // Commit and push
    println!("Committing and pushing changes...");
    let commit_msg = format!("Add bitstream for source MD5: {}", md5_hash);
    commit_and_push(&repo_dir, &commit_msg)?;

    println!("Successfully published bitstream with MD5: {}", md5_hash);

    Ok(())
}

/// Handle the get subcommand
fn handle_get(repo: &str, md5: &str) -> io::Result<()> {
    println!("Retrieving bitstream for MD5: {}", md5);

    // Create temporary directory for repository
    let temp_dir = tempfile::tempdir()?;
    let repo_dir = temp_dir.path().join("repo");

    // Clone repository
    println!("Cloning repository: {}", repo);
    clone_repository(repo, &repo_dir)?;

    // Load metadata
    let metadata_path = repo_dir.join("bitcache_metadata.json");
    if !metadata_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Metadata file not found in repository",
        ));
    }

    let metadata = Metadata::load_from_file(&metadata_path)?;

    // Find entry by MD5
    let entry = metadata.entries.get(md5).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("No binary found for MD5: {}", md5),
        )
    })?;

    // Get binary file path
    let binary_path = repo_dir.join(&entry.binary_path);
    if !binary_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Binary file not found: {}", entry.binary_path),
        ));
    }

    // Copy to current directory
    let filename = binary_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary path"))?;

    let current_dir = std::env::current_dir()?;
    let dest_path = current_dir.join(filename);

    println!("Copying {} to {}", filename.to_string_lossy(), dest_path.display());
    fs::copy(&binary_path, &dest_path)?;

    println!("Successfully retrieved bitstream:");
    println!("  Source file: {}", entry.source_file);
    println!("  MD5: {}", entry.md5);
    println!("  Timestamp: {}", entry.timestamp);
    println!("  Saved to: {}", dest_path.display());

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Publish {
            repo,
            source,
            bitstream,
            path,
        } => handle_publish(&repo, &source, &bitstream, &path),
        Commands::Get { repo, md5 } => handle_get(&repo, &md5),
    }
}
