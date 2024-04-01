use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Parent directory of all git repos
    #[arg(short, long, value_parser, default_value = ".")]
    dir: String,

    /// Depth to recursively search subfolders for git repositories
    #[arg(short, long, value_parser, default_value = "4")]
    max_depth: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let repo_paths = find_git_repos(&args.dir, args.max_depth)?;
    print_results(&repo_paths);

    Ok(())
}

fn find_git_repos(parent_directory: &str, max_depth: usize) -> Result<Vec<PathBuf>> {
    WalkDir::new(parent_directory)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.with_context(|| {
                format!("Failed to read directory entry in '{}'", parent_directory)
            });
            match entry {
                Ok(entry) if is_git_directory(entry.path()) => Some(Ok(entry.into_path())),
                Ok(_) => None, // Ignore non-git directories without errors
                Err(e) => Some(Err(e.into())),
            }
        })
        .collect()
}

fn is_git_directory(path: &Path) -> bool {
    path.is_dir() && path.join(".git").exists()
}

fn print_results(repo_paths: &[PathBuf]) {
    for path in repo_paths {
        println!("{}", path.display());
    }
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
