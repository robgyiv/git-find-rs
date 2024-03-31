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
