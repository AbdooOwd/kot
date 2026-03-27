mod cli;
mod utils;

use std::{
    env, fs::{self, DirEntry, read_link}, os::unix::fs::symlink, path::{Path, PathBuf}, process::exit
};
use cli::KotArgs;
use clap::Parser;

/*
*   --- EXIT CODES ---
*   1: created symlinks for content of cwd into parent (no args provided)
*
*   # ERRORS
*   -1: Not on unix-like platform
* */

fn main() {
    if !utils::platform_is_unix(env::consts::OS.to_string()) {
        eprintln!("Platform not Unix-like!");
        exit(-1);
    }

    let args = KotArgs::parse();
    let argc = env::args().len();

    if argc < 2 {
        // copy the current directories content into the parent directory
        let cur_dir = env::current_dir().expect("Couldn't read current directory");
        let dir_entries = cur_dir.read_dir()
            .expect("Couldn't read directory's entries");

        for entry in dir_entries {
            if entry.is_err() {
                continue;
            }

            let entry = entry.unwrap();
            let parent_dir = cur_dir.parent().expect("Couldn't get parent dir");
            create_symlink(entry.path(), parent_dir.to_path_buf());
        }

        exit(1);
    }

}

/// Creates a symlink to a file `target` with the same filename in `path`
fn create_symlink(target: PathBuf, path: PathBuf) {
    let new_symlink_path = path.join(target.file_name()
        .expect("Couldn't get target's file name"));

    if new_symlink_path.exists() {
        println!("A file of the same path and name already exists!");

        if new_symlink_path.is_symlink() {
            let existing_symlink_target_path = read_link(&new_symlink_path)
                .expect("Couldn't read existing symlink!");
            println!("Already existing file is a symlink pointing to '{}'",
                existing_symlink_target_path.to_str().unwrap());
            if existing_symlink_target_path != new_symlink_path {
                println!("Existing symlink does not point to the file intended to symlink");
                println!("-> Old symlink points to: {}", existing_symlink_target_path.to_str().unwrap());
                println!("-> New symlink (should) point to: {}", target.to_str().unwrap());
            } else {
                println!("Symlink points to the same entry!");
            }
        } else {
            println!("File is NOT a symlink")
        }
    } else {
        symlink(&target, new_symlink_path).expect("Couldn't create symlink!");
        println!("Created symlink to {}", target.to_str().unwrap());
    }
}
