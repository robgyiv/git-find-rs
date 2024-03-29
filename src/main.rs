use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("Parent directory of all repos not specified - try 'git-find-rs $HOME/code'");
    }

    let parent_directory = &args[1];
    let repo_paths = find_git_repos(parent_directory)?;
    print_results(&repo_paths);

    Ok(())
}

fn find_git_repos(parent_directory: &str) -> Result<Vec<PathBuf>> {
    WalkDir::new(parent_directory)
        .max_depth(6)
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
