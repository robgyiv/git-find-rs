use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    walk_directories();
}

fn walk_directories() -> () {
    let args: Vec<String> = env::args().collect();
    let code_directory = &args[1];
    let mut it = WalkDir::new(code_directory).max_depth(3).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if has_git_directory(&entry) {
            let path = Path::new(entry.path());
            println!("{}", path.display());
            it.skip_current_dir();
            continue;
        }
    }
}

fn has_git_directory(entry: &DirEntry) -> bool {
    Path::new(&entry.path().join(".git")).exists()
}
