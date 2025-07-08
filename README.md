# Alternative history for your git repo
<img src="logo.svg" alt="alt - Alternative history for your git repo"><br>

## How it works
`alt` creates a dir where it can store your git repo and replace it with another one, so you can have different repos tracking the same files, with _completely different_ commits, branches, remotes, stash, etc.

Yes, I know this is nearly the same as branches, but i thought it might be an interesting project, and there are some differences.

As far as I know, `alt` should be compatible any other git-related tools - they will only see the currently active repo.

### Warning ⚠️
This is a work in progress. Use at your own risk.

## Usage
```
$ alt --help
Alternative history for your git repo

Usage: alt [OPTIONS] <COMMAND>

Commands:
  switch  switch repos
  list    list stored repos
  rename  rename a repo
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>          prefix to alt path and git path [default: ./]
  -a, --alt-path <ALT_PATH>  path to alt dir, relative to PATH [default: .alt/]
  -g, --git-path <GIT_PATH>  path to git dir, relative to PATH [default: .git/]
  -s, --setup                create new alt dir
  -h, --help                 Print help
  -V, --version              Print version
```

### Example (edited for brevity & clarity)
```
$ git status
On branch master

Changes not staged for commit:
        modified:   README.md

$ git log
commit ... (HEAD -> master, gh/master, gh/HEAD)
...

$ alt list
  public
* private

$ alt switch public
Storing active repo as private

$ git status
On branch master

No commits yet

Untracked files:
        .gitignore
        ...

$ git log
fatal: your current branch 'master' does not have any commits yet
```

## Help
This is my first "real" project in Rust (and in open source), so if you have any advice / improvements I would greatly appreciate it.

## TODO
- [ ] `alt switch` with no args should work if there are only two repos and one is active
  - [ ] same behavior for `alt` with no subcommand
- [ ] write tests
- [ ] `alt extras` subcommand to add extra files to `switch` other than provided git path
