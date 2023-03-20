# git-find-rs

A CLI tool used for finding git repositories.

I've been using [git-find-repos](https://github.com/acroz/git-find-repos) every day for a long time, so it seemed like a good idea to [Rewrite it in Rust](https://transitiontech.ca/random/RIIR) to learn the language.

Note that as of right now, this is mostly for my own understanding and built for my own use case.

---

## Benchmarks

Obviously this is not a totally fair comparison as Rust is compiled, but compared to [git-find-repos](https://github.com/acroz/git-find-repos) we see the following improvement on my 2017 MacBook Pro with 100 git directories:

```bash
Benchmark 1: /usr/local/bin/git-find-rs $HOME/code
  Time (mean ± σ):     596.9 ms ±  29.4 ms    [User: 112.5 ms, System: 474.0 ms]
  Range (min … max):   551.2 ms … 633.7 ms    10 runs
 
Benchmark 2: /Users/robbie/.local/bin/git-find-repos $HOME/code
  Time (mean ± σ):     918.9 ms ±  99.6 ms    [User: 242.7 ms, System: 651.4 ms]
  Range (min … max):   791.6 ms … 1044.4 ms    10 runs
 
Summary
  '/usr/local/bin/git-find-rs $HOME/code' ran
    1.54 ± 0.18 times faster than '/Users/robbie/.local/bin/git-find-repos $HOME/code'
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
