use clap::{Args, Parser, Subcommand };

#[derive(Parser)]
#[command(author, version, about)]
pub struct KotArgs {
    /// The directory to symlink
    pub src: Option<String>,
    /// The directory in which the symlink should be placed
    pub dest: Option<String>
}
