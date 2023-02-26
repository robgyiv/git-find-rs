use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    unsafe { walk_directories() };
}

unsafe fn walk_directories() -> () {
    let args: Vec<String> = env::args().collect();
    let code_directory = &args[1];
    let mut it = WalkDir::new(code_directory).max_depth(3).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if is_git_directory(&entry) {
            let path = Path::new(entry.path());
            let parent = path.parent().unwrap();
            println!("{}", parent.display());
            it.skip_current_dir();
            continue;
        }
    }
}

fn is_git_directory(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".git"))
        .unwrap_or(false)
}
