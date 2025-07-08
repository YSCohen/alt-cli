use crate::fs_helpers;
use snafu::{Whatever, prelude::*};
use std::path::Path;
use stdext::function_name;

/// Rename alt repos
pub fn rename(alt_path: &Path, from: &String, to: &String) -> Result<(), Whatever> {
    fs_helpers::rename(alt_path, from, to).with_whatever_context(|_| {
        format!(
            "{}: Could not rename {from:?} to {to:?} in alt dir {alt_path:?}",
            function_name!()
        )
    })
}
