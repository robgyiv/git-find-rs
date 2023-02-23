use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    unsafe { dirs() };
}

unsafe fn dirs() -> () {
    let mut it = WalkDir::new("/Users/robbie/code").max_depth(3).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if is_git_directory(&entry) {
            it.skip_current_dir();
            let path = Path::new(entry.path());
            let parent = path.parent().unwrap();
            println!("{}", parent.display());
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
