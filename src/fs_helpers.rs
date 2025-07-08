use snafu::{Whatever, prelude::*};
use std::{
    fs,
    path::{Path, PathBuf},
};
use stdext::function_name;

static ACTIVE_FILE: &str = "_ALT_ACTIVE";

/// create the alt dir to store the config and switched-out repos
pub fn create_alt(alt_path: &PathBuf) -> Result<(), Whatever> {
    let active_file_path = alt_path.join(ACTIVE_FILE);
    fs::create_dir(alt_path).with_whatever_context(|_| {
        format!("{}: Could not create {alt_path:?}", function_name!())
    })?;
    fs::write(&active_file_path, "").with_whatever_context(|_| {
        format!(
            "{}: Could not create {active_file_path:?}",
            function_name!()
        )
    })
}

/// move (active) repo from `git_path` to `alt_path`
pub fn store(repo_name: &PathBuf, alt_path: &Path, git_path: &PathBuf) -> Result<(), Whatever> {
    let dest_full_path = alt_path.join(repo_name);
    fs::rename(git_path, &dest_full_path).with_whatever_context(|_| {
        format!(
            "{}: Could not rename {git_path:?} to {dest_full_path:?}",
            function_name!()
        )
    })?;
    set_active(alt_path, &String::new())
        .with_whatever_context(|_| format!("{}: Could not clear active repo", function_name!()))
}

/// move a repo from `alt_path` to `git_path`
pub fn retrieve(repo_name: &String, alt_path: &Path, git_path: &PathBuf) -> Result<(), Whatever> {
    let src_full_path = alt_path.join(repo_name);
    fs::rename(&src_full_path, git_path).with_whatever_context(|_| {
        format!(
            "{}: Could not rename {src_full_path:?} to {git_path:?}",
            function_name!()
        )
    })?;
    fs::create_dir(&src_full_path).with_whatever_context(|_| {
        format!("{}: Could not create {src_full_path:?}", function_name!())
    })?;
    set_active(alt_path, repo_name).with_whatever_context(|_| {
        format!(
            "{}: Could not set {repo_name:?} as active repo",
            function_name!()
        )
    })
}

/// rename a repo
pub fn rename(alt_path: &Path, from: &String, to: &String) -> Result<(), Whatever> {
    let src_full_path = alt_path.join(from);
    let dest_full_path = alt_path.join(to);
    fs::rename(&src_full_path, &dest_full_path).with_whatever_context(|_| {
        format!(
            "{}: Could not rename {src_full_path:?} to {dest_full_path:?}",
            function_name!()
        )
    })
}

/// return vector of tuple `(dir: PathBuf, is_active:bool)` representing the subdirs of `alt_path`
pub fn subdirs_active(alt_path: &PathBuf) -> Result<Vec<(PathBuf, bool)>, Whatever> {
    let child_dirs = get_child_dirs(alt_path).with_whatever_context(|_| {
        format!(
            "{}: Could not get child dirs of {alt_path:?}",
            function_name!()
        )
    })?;
    let mut path_and_active: Vec<(PathBuf, bool)> = Vec::new();
    let active_name = get_active(alt_path)
        .with_whatever_context(|_| {
            format!(
                "{}: Could not get active repo of {alt_path:?}",
                function_name!()
            )
        })?
        .unwrap_or_default();

    for dir in child_dirs {
        let dir_full_path = alt_path.join(&dir);
        let is_active = match dir_full_path.file_name() {
            Some(name) => active_name == name.to_str().unwrap_or_default(),
            None => false,
        };
        path_and_active.push((dir, is_active));
    }
    Ok(path_and_active)
}

/// get child dirs of dir at `path` as a Vec<PathBuf>
fn get_child_dirs(path: &PathBuf) -> Result<Vec<PathBuf>, Whatever> {
    let mut child_dirs = Vec::new();
    let dir_contents = fs::read_dir(path)
        .with_whatever_context(|_| format!("{}: Could not read {path:?}", function_name!()))?;

    for entry in dir_contents {
        let entry = entry.with_whatever_context(|_| {
            format!(
                "{}: Could not iterate children of {path:?}",
                function_name!()
            )
        })?;
        let metadata = entry.metadata().with_whatever_context(|_| {
            format!("{}: Could not get metadata of {entry:?}", function_name!())
        })?;
        if metadata.is_dir() {
            child_dirs.push(PathBuf::from(entry.file_name()));
        }
    }

    Ok(child_dirs)
}

/// set active repo name
fn set_active(alt_path: &Path, name: &String) -> Result<(), Whatever> {
    let active_file_path = alt_path.join(ACTIVE_FILE);
    fs::write(&active_file_path, name).with_whatever_context(|_| {
        format!(
            "{}: Could not write {name:?} to {active_file_path:?}",
            function_name!()
        )
    })
}

/// get active repo name
pub fn get_active(alt_path: &Path) -> Result<Option<String>, Whatever> {
    let active_file_path = alt_path.join(ACTIVE_FILE);
    let contents = fs::read_to_string(&active_file_path).with_whatever_context(|_| {
        format!("{}: Could not read {active_file_path:?}", function_name!())
    })?;
    if contents.is_empty() {
        Ok(None)
    } else {
        Ok(Some(contents))
    }
}
