use clap::{Parser, Subcommand};

use std::path::PathBuf;

/// Alternative history for your git repo
#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    /// prefix to alt path and git path
    #[arg(short, long, default_value = "./")]
    pub path: PathBuf,

    /// path to alt dir, relative to PATH
    #[arg(short, long, default_value = ".alt/")]
    pub alt_path: PathBuf,

    /// path to git dir, relative to PATH
    #[arg(short, long, default_value = ".git/")]
    pub git_path: PathBuf,

    /// create new alt dir
    #[arg(short, long)]
    pub setup: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// switch repos
    Switch {
        /// name of repo to switch to
        name: Option<String>,

        /// instead of retrieving an existing repo, init a new one
        #[arg(short, long)]
        init: bool,
    },
    /// list stored repos
    List {},
    /// rename a repo
    #[command(arg_required_else_help = true)]
    Rename {
        /// current name of repo
        from: String,
        /// name to move to
        to: String,
    },
}
