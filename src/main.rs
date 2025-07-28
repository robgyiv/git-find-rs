use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const GIT_DIR: &str = ".git";
const DEFAULT_SEARCH_PATH: &str = ".";
const DEFAULT_MAX_DEPTH: &str = "4";
const INITIAL_CAPACITY: usize = 64;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Parent directory of all git repos
    #[arg(short = 'd', long = "dir", value_parser, default_value = DEFAULT_SEARCH_PATH)]
    search_path: String,

    /// Depth to recursively search subfolders for git repositories
    #[arg(short, long, value_parser, default_value = DEFAULT_MAX_DEPTH)]
    max_depth: usize,
}

fn main() -> Result<()> {
    run()
}

/// Main application logic separated from CLI entry point.
fn run() -> Result<()> {
    let args = Args::parse();

    let repo_paths = find_git_repos(&args.search_path, args.max_depth)?;
    print_results(&repo_paths)?;

    Ok(())
}

/// Recursively searches for git repositories in the specified directory.
///
/// # Arguments
/// * `search_path` - The root directory to search from
/// * `max_depth` - Maximum depth to recurse into subdirectories
///
/// # Returns
/// A vector of paths to directories containing .git folders
///
/// # Errors
/// Returns an error if directory traversal fails due to permissions or I/O issues
fn find_git_repos(search_path: &str, max_depth: usize) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::with_capacity(INITIAL_CAPACITY); // Initial capacity hint for common cases
    
    for entry in WalkDir::new(search_path)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Skip .git subdirectories but allow .git directories themselves
            !(e.file_name() == GIT_DIR && e.depth() > 0)
        })
    {
        let entry = entry.with_context(|| {
            format!("Failed to read directory entry")
        })?;
        
        if is_git_directory(entry.path()) {
            repos.push(entry.into_path());
        }
    }
    
    Ok(repos)
}

/// Checks if a directory is a git repository by looking for a .git folder.
///
/// # Arguments
/// * `path` - The directory path to check
///
/// # Returns
/// `true` if the path is a directory containing a .git folder, `false` otherwise
fn is_git_directory(path: &Path) -> bool {
    path.join(GIT_DIR).exists() && path.is_dir()
}

/// Prints the list of repository paths to stdout using buffered output.
///
/// # Arguments
/// * `repo_paths` - A slice of PathBuf objects representing git repository paths
///
/// # Returns
/// `Ok(())` on success, or an error if writing to stdout fails
///
/// # Errors
/// Returns an error if there are I/O issues writing to stdout
fn print_results(repo_paths: &[PathBuf]) -> Result<()> {
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    for path in repo_paths {
        writeln!(writer, "{}", path.display())?;
    }
    
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
    use std::io::Write;

    #[test]
    fn test_is_git_directory() {
        use super::is_git_directory;
        let temp_dir = tempfile::tempdir().unwrap();
        let git_dir = temp_dir.path().join(".git");
        create_dir(&git_dir).unwrap();
        let mut file = File::create(git_dir.join("HEAD")).unwrap();
        writeln!(file, "ref: refs/heads/main").unwrap();
        assert!(is_git_directory(&temp_dir.path()));
        remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_is_not_git_directory() {
        use super::is_git_directory;
        let temp_dir = tempfile::tempdir().unwrap();
        assert!(!is_git_directory(&temp_dir.path()));
        remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_find_git_repos() {
        use super::find_git_repos;
        let temp_dir = tempfile::tempdir().unwrap();
        let git_dir = temp_dir.path().join(".git");
        create_dir(&git_dir).unwrap();
        let repos = find_git_repos(temp_dir.path().to_str().unwrap(), 1).unwrap();
        assert_eq!(repos.len(), 1);
        remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_find_no_git_repos() {
        use super::find_git_repos;
        let temp_dir = tempfile::tempdir().unwrap();
        let repos = find_git_repos(temp_dir.path().to_str().unwrap(), 1).unwrap();
        assert!(repos.is_empty());
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_find_git_repos_no_results() {
        use super::find_git_repos;
        let temp_dir = tempfile::tempdir().unwrap();
        let repos = find_git_repos(temp_dir.path().to_str().unwrap(), 1);
        assert!(repos.expect("Failed to read directory entry in").is_empty());
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_find_get_repos_max_depth_no_results() {
        use super::find_git_repos;
        let temp_dir = tempfile::tempdir().unwrap();
        let git_dir = temp_dir.path().join("outside-search-depth/.git");
        create_dir_all(&git_dir).unwrap();
        let repos = find_git_repos(temp_dir.path().to_str().unwrap(), 0).unwrap();
        assert!(repos.is_empty());
        remove_dir_all(temp_dir).unwrap();
    }
}
