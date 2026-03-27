use std::{fs::DirEntry, path::PathBuf, process::Command};

/// Works on files & dirs cuz it uses the "cp" command
pub fn copy_stuff(dir: DirEntry, dest: PathBuf) {
    Command::new("cp")
        .args([
            "-r",
            dir.path().to_str().expect("Couldn't convert src dir path to str"),
            dest.to_str().expect("Couldn't convert dest dir path to str")
        ])
        .spawn().expect("Couldn't spawn 'cp' command to copy content!")
        .wait().expect("Couldn't wait for 'cp' command");
}

pub fn platform_is_unix(os_name: String) -> bool {
    matches!(os_name.as_str(), "linux" | "macos" | "freebsd" | "openbsd" |
        "dragonfly" | "solaris" | "illumos" | "aix" |
        "hurd" | "redox" | "haiku")
}
