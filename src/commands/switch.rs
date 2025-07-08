use crate::fs_helpers;
use git2::Repository;
use snafu::{Whatever, prelude::*};
use std::{iter, path::PathBuf};
use stdext::function_name;

/// If `git_path` exists, store it to the active (empty) dir in `alt_path`.
/// Then optionally retrieve another repo from `alt_path`,
/// or init a new repo at `base_path`
pub fn switch(
    base_path: &PathBuf,
    alt_path: &PathBuf,
    git_path: &PathBuf,
    retrieve_name: &Option<String>,
    git_init: &bool,
) -> Result<(), Whatever> {
    ensure_whatever!(
        alt_path.exists(),
        "{}: alt dir does not exist, supposedly at {alt_path:?}",
        function_name!()
    );

    // there is an active repo to store. otherwise skip all store-related actions
    let store: bool = git_path.exists();

    let mut store_path = PathBuf::new();

    if store {
        store_path = PathBuf::from(
            match fs_helpers::get_active(alt_path).with_whatever_context(|_| {
                format!("{}: Could not get_active of {alt_path:?}", function_name!())
            })? {
                // there is an active repo
                Some(path) => path,

                // otherwise, generate random name for new repo
                None => iter::repeat_with(fastrand::alphanumeric)
                    .take(10)
                    .collect::<String>(),
            },
        );
        println!(
            "Storing active repo as {}",
            store_path.file_name().unwrap_or_default().display()
        );
    }

    // should be impossible for `store_path` to be empty if `store`
    ensure_whatever!(
        !(store && store_path.as_os_str().is_empty()),
        "{}: store is true, but store_path is empty",
        function_name!()
    );

    match (retrieve_name, git_init, store) {
        (Some(_), true, _) => whatever!("Can't switch to a repo and init a new repo"),
        (Some(name), false, true) => {
            fs_helpers::store(&store_path, alt_path, git_path).with_whatever_context(|_| {
                format!(
                    "{}: Could not store {git_path:?} to {store_path:?}",
                    function_name!()
                )
            })?;
            fs_helpers::retrieve(name, alt_path, git_path).with_whatever_context(|_| {
                format!(
                    "{}: Could not retrieve {name:?} from {alt_path:?} to {git_path:?}",
                    function_name!()
                )
            })
        }
        (Some(name), false, false) => fs_helpers::retrieve(name, alt_path, git_path)
            .with_whatever_context(|_| {
                format!(
                    "{}: Could not retrieve {name:?} from {alt_path:?} to {git_path:?}",
                    function_name!()
                )
            }),
        (None, true, true) => {
            fs_helpers::store(&store_path, alt_path, git_path).with_whatever_context(|_| {
                format!(
                    "{}: Could not store {git_path:?} to {store_path:?}",
                    function_name!()
                )
            })?;
            Repository::init(base_path).with_whatever_context(|_| {
                format!("{}: Could not git init at {base_path:?}", function_name!())
            })?;
            Ok(())
        }
        (None, true, false) => whatever!("{}: Just do a `git init` yourself!", 13),
        (None, false, true) => fs_helpers::store(&store_path, alt_path, git_path)
            .with_whatever_context(|_| {
                format!(
                    "{}: Could not store {git_path:?} to {store_path:?}",
                    function_name!()
                )
            }),
        (None, false, false) => whatever!("Nothing to do"),
    }
}
