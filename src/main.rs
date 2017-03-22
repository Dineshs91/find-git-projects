use std::thread;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;


fn main() {
    let dirs = fs::read_dir("/Users/dinesh/Documents/developer/").unwrap();

    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_path = dir_entry.path();
        let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

        if is_dir {
            walk(dir_path);
        }
    }
}

fn walk(dir_path: PathBuf) {
    // We will use threads only at this level.
    if find_git(&dir_path) {
        println!("Path is {:?}", dir_path);
    } else {
        let handle = thread::spawn(|| {
            let dirs = fs::read_dir(dir_path).unwrap();

            for dir in dirs {
                let dir_entry = dir.unwrap();
                let dir_path = dir_entry.path();
                let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

                if is_dir {
                    walk(dir_path);
                }
            }
        });
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
