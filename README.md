# Kot - A GNU-Stow Alternative

Kot is a "symlink management" tool written in Rust that aims to be
a modern `stow` for Unix-like systems (so far).

## The Issue Kot Fixes

Let's imagine a file structure:
```
~/
├── .config/
└── dotfiles/
    └── .config/
        ├── awesome/
        ├── some-config/
        └── alacritty/
```

I prefer having my dotfiles repo at `~/dotfiles/`. It simulates the home
directory by containing `.config` or anything else.
If I used stow by doing `stow .config`, it would try to create a symlink
for `.config` in `~/` (the parent directory of `dotfiles/`). It will then fail
because a directory of the same name already exists (assuming we're talking about
a Unix-like system).

Kot aims to, instead of creating a symlink for a certain directory, create symlinks
of the **sub-directories**. So for example, if I tried to use Kot on the earlier
directory `.config`, the result would be:

```
~/
├── .config/
│   ├── ~awesome/
│   ├── ~some-config/
│   └── ~alacritty/
└── dotfiles/
    └── .config/
        ├── awesome/
        ├── some-config/
        └── alacritty/
```
> `~` in symlink filenames just for explanation
