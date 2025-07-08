use clap::Parser;
use snafu::{Whatever, prelude::*};

mod cli;
mod commands;
mod fs_helpers;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let args = cli::Cli::parse();

    let alt_full_path = args.path.join(&args.alt_path);
    let git_full_path = args.path.join(&args.git_path);

    if args.setup {
        fs_helpers::create_alt(&alt_full_path)
            .with_whatever_context(|_| format!("Could not create alt dir at {alt_full_path:?}"))?;
        println!("alt dir created at {alt_full_path:?}");
    }

    match &args.command {
        cli::Commands::Switch { name, init } => {
            commands::switch::switch(&args.path, &alt_full_path, &git_full_path, name, init)?
        }
        cli::Commands::List {} => commands::list::list(&alt_full_path)?,
        cli::Commands::Rename {
            from: src,
            to: dest,
        } => commands::rename::rename(&alt_full_path, src, dest)?,
    };
    Ok(())
}
