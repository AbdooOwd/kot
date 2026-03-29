mod cli;
mod utils;

use std::{
    env,
    fs::{ read_link, canonicalize },
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    sync::RwLock
};
use cli::KotArgs;
use utils::our_exit;
use clap::Parser;

/*
*   --- EXIT CODES ---
*   0: created symlinks with provided `src` and `dest` args
*   1: created symlinks for content of cwd into parent (no args provided)
*
*   # ERRORS
*   -1: Not on unix-like platform
*   -2: 'source' argument provided isn't a directory
* */


static DEBUG_LOGS: RwLock<bool> = RwLock::new(false);

fn main() {
    if !utils::platform_is_unix(env::consts::OS.to_string()) {
        eprintln!("Platform not Unix-like!");
        our_exit(-1);
    }

    let args = KotArgs::parse();

    if args.src.is_none() || args.dest.is_none() {
        // make symlinks for content of current dir in parent directory
        let cur_dir = env::current_dir().expect("Couldn't read current directory");
        let dir_entries = cur_dir.read_dir()
            .expect("Couldn't read directory's entries");

        for entry in dir_entries {
            if entry.is_err() {
                // for some reasons dir's entry doesn't exist anymore??? (just to be safe)
                continue;
            }

            let entry = entry.unwrap();
            let parent_dir = cur_dir.parent().expect("Couldn't get parent dir");
            create_symlink(entry.path(), parent_dir.to_path_buf(), false);
        }

        our_exit(1);
    }

    // we must first turn the provided paths into realpaths
    // (ps: I DONT WANNA CLONE WAAAHHHHH)
    let src: PathBuf = match args.src.clone().unwrap() == "." {
        true => env::current_dir().expect("Coudln't read current working directory (cwd)"),
        false => PathBuf::from(args.src.unwrap())
    };
    let dest = PathBuf::from(args.dest.unwrap());
    *DEBUG_LOGS.write().unwrap() = args.debug_logs;

    kot_dir(&src, &dest);

    println!("Everything SHOULD have gone right");
    our_exit(0);
}

/// Creates a symlink to a file `target` with the same filename in `path`
fn create_symlink(target: PathBuf, path: PathBuf, ignore_existing_dir: bool) {
    let target = target.canonicalize().unwrap();
    let path = path.canonicalize().unwrap();

    if target.file_name().unwrap().to_str().unwrap() == ".git" && target.is_dir() {
        println!("Ignoring '.git/'");
        return;
    }

    let target_filename = target.file_name()
        .unwrap_or_else(|| panic!("Couldn't get target's file name for: '{}'", target.to_str().unwrap()));
    let new_symlink_path = path.join(target_filename);

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
            debug_log!("File is NOT a symlink");

            // in order to be able to be recursive without looping forever
            if !ignore_existing_dir {
                // with a max depth of 1, look through the directory
                // we want to symlink and symlink what's inside instead
                if target.is_dir() {
                    println!("Already existing entry is a directory\nProceeding into symlinking sub-entries...");

                    // we'll look inside the current target which should be
                    // a directory. Then we symlink every entry of this
                    // directory
                    let entries = target.read_dir().expect("Couldn't read sub-entries!");
                    for entry in entries {
                        // sub-entries of `target` dir to symlink
                        if entry.is_err() {
                            continue;
                        }

                        let entry = entry.unwrap();
                        // destination path, but with the sub-directory we're symlinking
                        debug_log!("Gonna symlink entry: '{}' into '{}'",
                            entry.path().to_str().unwrap(),
                            new_symlink_path.to_str().unwrap());
                        // we get the new path in which we should place the sub-entries' symlinks
                        // we'll ignore existing dir in order to not be infinitely recursive
                        create_symlink(entry.path(), new_symlink_path.to_path_buf(), true);
                    }
                } else {
                    println!("Existing entry isn't a directory, thus aborting");
                }
            } else {
                debug_log!("Not gonna create symlink for sub-entries of directory '{}'", target.to_str().unwrap());
            }
        }
    } else {
        debug_log!("Creating symlink for '{}' as '{}'", target.to_str().unwrap(), new_symlink_path.to_str().unwrap());
        debug_log!("New symlink expected to be at {}", new_symlink_path.to_str().unwrap());

        symlink(&target, &new_symlink_path).expect("Couldn't create symlink!");

        println!("Created symlink to '{}' in '{}'",
            target.to_str().unwrap(), new_symlink_path.to_str().unwrap());
    }
}

/// `src` is the directory in which are the entries TO symlink.
/// `src` itself won't be symlinked, but its content, then into
/// the `dest` directory
fn kot_dir(src: &Path, dest: &Path) {
    if !src.is_dir() {
        println!("Provided 'source' path is not a directory. aborting");
        our_exit(-2);
    }

    let src_entries = src.read_dir().unwrap_or_else(|x| panic!("Couldn't read source directory '{}'\n Error: {}",
        src.to_str().unwrap(), x));
    for entry in src_entries {
        let entry = entry.unwrap();
        debug_log!("Current entry: {}", entry.path().to_str().unwrap());
        create_symlink(entry.path(), dest.to_path_buf(), false);
    }
}
