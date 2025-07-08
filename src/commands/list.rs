use crate::fs_helpers;
use snafu::{Whatever, prelude::*};
use std::path::PathBuf;
use stdext::function_name;

/// List all repos stored in alt dir
pub fn list(alt_path: &PathBuf) -> Result<(), Whatever> {
    let mut repos = Vec::new();
    for (dir, is_active) in fs_helpers::subdirs_active(alt_path).with_whatever_context(|_| {
        format!(
            "{}: Could not get subdirs of {alt_path:?}",
            function_name!()
        )
    })? {
        repos.push(format!(
            "{}{}",
            if is_active { "* " } else { "  " },
            dir.file_name().unwrap_or_default().display()
        ))
    }
    println!("{}", repos.join("\n"));
    Ok(())
}
