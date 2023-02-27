# git-find-rs

A CLI tool used for finding git repositories.

I've been using [git-find-repos](https://github.com/acroz/git-find-repos) every day for a long time, so it seemed like a good idea to [Rewrite it in Rust](https://transitiontech.ca/random/RIIR) to learn the language.

Note that as of right now, this is mostly for my own understanding and built for my own use case.

---

## Installation

```bash
# Clone this repo to a local directory, for example `/Users/foo/git-find-rs`
$ cargo build --release
$ ln -s /Users/foo/git-find-rs/target/release/git-find-rs /usr/local/bin/git-find-rs
```

---

## Usage with fish shell

Create a function to use `git-find-rs`, for example:

```bash
function repo
    set initial_query $argv
    set code_dir "/Users/foo/code"
    set dest (git-find-rs "$code_dir" | fzy -q "$initial_query" -l 20) && cd "$dest"
end
```

You can then:

```bash
$ repo my-repo-name
```

This will show a list of git directories. You can type a partial string of the repo name and `fzy` will match this for you. Selecting a directory will `cd` into the directory.

---

## Todo

- Tests
- crates.io release
