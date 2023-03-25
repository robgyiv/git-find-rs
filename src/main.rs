use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let parent_directory = &args[1];
        let repo_paths: Vec<String> = walk_directories(parent_directory.to_string());
        print_results(repo_paths);
    } else {
        println!("Repo parent directory not specified - try 'git-find-rs /home/foo/code'");
    }
}

fn walk_directories(parent_directory: String) -> Vec<String> {
    let mut repo_paths: Vec<String> = Vec::new();
    let mut it = WalkDir::new(parent_directory).max_depth(6).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if has_git_directory(&entry) {
            let path = Path::new(entry.path());
            repo_paths.push(path.display().to_string());
            it.skip_current_dir();
            continue;
        }
    }
    return repo_paths;
}

fn has_git_directory(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return Path::new(&entry.path().join(".git")).exists();
    }
    return false;
}

fn print_results(repo_paths: Vec<String>) -> () {
    repo_paths.iter().for_each(|it| {
        println!("{}", it);
    })
}
