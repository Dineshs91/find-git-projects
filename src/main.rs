use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

// Go to the development directory. Find out and list all the
// directories that have git in them.
//
// Phase2: Do it concurrently.
fn main() {
    // Basic approach.
    // Start from the root directory.
    // Recursively traverse all the child directories looking for .git
    // If found then don't recurse further into its descendants.
    // Else continue going deep.
    let dirs = fs::read_dir("/Users/dinesh/Documents/developer/").unwrap();

    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_name = dir_entry.file_name();
        let dir_path = dir_entry.path();
        let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

        if is_dir {
            //println!("{:?} - {:?}", dir_name, dir_path);
            walk(dir_path);
        }
    }
}

fn walk(dir_path: PathBuf) {
    if find_git(&dir_path) {
        println!("Path is {:?}", dir_path);
    } else {
        let dirs = fs::read_dir(dir_path).unwrap();

        for dir in dirs {
            let dir_entry = dir.unwrap();
            let dir_path = dir_entry.path();
            let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

            if is_dir {
                walk(dir_path);
            }
        }
    }

}

fn find_git(dir_path: &PathBuf) -> bool {
    let dir_files = fs::read_dir(dir_path).unwrap();

    for file in dir_files {
        let file_name = file.unwrap().file_name();

        if file_name == OsStr::new(".git") {
            return true
        }
    }

    false
}
