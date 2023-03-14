# git-find-rs

A CLI tool used for finding git repositories.

I've been using [git-find-repos](https://github.com/acroz/git-find-repos) every day for a long time, so it seemed like a good idea to [Rewrite it in Rust](https://transitiontech.ca/random/RIIR) to learn the language.

Note that as of right now, this is mostly for my own understanding and built for my own use case.

---

## Benchmarks

Obviously this is not a totally fair comparison as Rust is compiled, but compared to [git-find-repos](https://github.com/acroz/git-find-repos) we see the following improvement on my 2017 MacBook Pro with 100 git directories:

```bash
$ hyperfine --warmup 3 '/usr/local/bin/git-find-rs $HOME/code' '/usr/local/bin/git-find-repos $HOME/code'
Benchmark 1: /usr/local/bin/git-find-rs $HOME/code
  Time (mean ± σ):       9.0 ms ±   1.9 ms    [User: 2.0 ms, System: 5.6 ms]
  Range (min … max):     5.8 ms …  17.1 ms    198 runs

Benchmark 2: /usr/local/bin/git-find-repos $HOME/code
  Time (mean ± σ):     915.6 ms ±  27.9 ms    [User: 259.0 ms, System: 635.7 ms]
  Range (min … max):   880.3 ms … 972.1 ms    10 runs

Summary
  '/usr/local/bin/git-find-rs $HOME/code' ran
  101.36 ± 21.32 times faster than '/usr/local/bin/git-find-repos $HOME/code'
```

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

- Go blazingly fast
- Tests
- crates.io release
