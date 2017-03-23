extern crate clap;
use clap::{Arg, App};

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

static GLOBAL_PROJECT_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;


fn main() {
    let input_dir = cli();

    if input_dir.is_empty() {
        panic!("Provide a directory path to search");
    }

    let dir_path = PathBuf::from(input_dir);

    walk(dir_path);

    println!("Project count: {}", GLOBAL_PROJECT_COUNT.load(Ordering::Relaxed));
}

fn cli() -> String {
    let matches = App::new("find-git-projects")
        .version("0.1.0")
        .author("Dineshs91 <dineshpy07@gmail.com>")
        .about("Find the projects that use git")
        .arg(Arg::with_name("dir")
        .short("d")
        .long("dir")
        .value_name("DIR")
        .takes_value(true)).get_matches();

    let dir = matches.value_of("dir");
    dir.unwrap_or("").to_string()
}

fn walk(dir_path: PathBuf) {
    if find_git(&dir_path) {
        GLOBAL_PROJECT_COUNT.fetch_add(1, Ordering::Relaxed);
        println!("Path is {:?}", dir_path);
    } else {
        let dirs = fs::read_dir(dir_path).unwrap();
        let dirs_collect = dirs.collect::<Vec<_>>();

        if dirs_collect.len() > 10 {
            // launch a thread only if there are more than 10 sub directories.
            let handle = thread::spawn(|| {
                for dir in dirs_collect {
                    let dir_entry = dir.unwrap();
                    let dir_path = dir_entry.path();
                    let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

                    if is_dir {
                        walk(dir_path);
                    }
                }
            });

            // Wait for the child thread to finish processing.
            handle.join();
        } else {
            // Handle it in the same thread.
            for dir in dirs_collect {
                let dir_entry = dir.unwrap();
                let dir_path = dir_entry.path();
                let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

                if is_dir {
                    walk(dir_path);
                }
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
