# git-find-rs

A CLI tool used for finding git repositories.

I've been using [git-find-repos](https://github.com/acroz/git-find-repos) every day for a long time, so it seemed like a good idea to [Rewrite it in Rust](https://transitiontech.ca/random/RIIR) to learn the language.

Note that as of right now, this is mostly for my own understanding and built for my own use case.

---

## Benchmarks

Obviously this is not a totally fair comparison as Rust is compiled, but compared to [git-find-repos](https://github.com/acroz/git-find-repos) we see the following improvement on my 2017 MacBook Pro with ~100 git directories:

```bash
$ hyperfine --warmup 3 '/usr/local/bin/git-find-rs $HOME/code' '$HOME/.local/bin/git-find-repos $HOME/code'
Benchmark 1: /usr/local/bin/git-find-rs $HOME/code
  Time (mean ± σ):     284.2 ms ±   6.8 ms    [User: 45.5 ms, System: 234.2 ms]
  Range (min … max):   274.1 ms … 297.4 ms    10 runs

Benchmark 2: $HOME/.local/bin/git-find-repos $HOME/code
  Time (mean ± σ):     761.3 ms ±  37.6 ms    [User: 203.5 ms, System: 542.5 ms]
  Range (min … max):   733.6 ms … 860.2 ms    10 runs

Summary
  '/usr/local/bin/git-find-rs $HOME/code' ran
    2.68 ± 0.15 times faster than '$HOME/.local/bin/git-find-repos $HOME/code'
```

---

## Installation

```bash
# Clone this repo to a local directory, for example `/Users/foo/git-find-rs`
$ cargo build --release
$ ln -s /Users/foo/git-find-rs/target/release/git-find-rs /usr/local/bin/git-find-rs
```

---

## Using in fish shell

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

- Go blazingly fast
- crates.io release
