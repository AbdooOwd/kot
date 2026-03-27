# Todo

- what sets Kot apart from stow is the fact that if we're trying to make
  a symlink, and a file/dir of the same name exists, we instead iterate inside
  the source directory (max depth 1?). For example, when trying to symlink
  a `.config/`, it will definitely already be existing. So we'll look inside
  and symlink what we find, so for example we'd find `alacritty/`--we'd symlink that!

  For now, like Stow, the program just ignores the creation of the symlink
  if a file/dir already exists...

- Make the provided arguments useful! So far, we're like stow;
  we just symlink what's in the current working directory into
  the parent's directory of cwd!

> So far, then; this is just a cheap Rust rip-off of GNU-Stow?
