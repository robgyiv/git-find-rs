# git-find-rs

[![CI](https://github.com/robgyiv/git-find-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/robgyiv/git-find-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/git-find-rs.svg)](https://crates.io/crates/git-find-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A CLI tool used for finding git repositories.

I've been using [git-find-repos](https://github.com/acroz/git-find-repos) every day for a long time, so it seemed like a good idea to [Rewrite it in Rust](https://transitiontech.ca/random/RIIR) to learn the language.

Note that as of right now, this is mostly for my own understanding and built for my own use case.

---

## Benchmarks

Obviously this is not a totally fair comparison as Rust is compiled, but compared to [git-find-repos](https://github.com/acroz/git-find-repos) we see the following improvement on my 2017 MacBook Pro with ~100 git directories:

```bash
$ hyperfine --warmup 3 '/usr/local/bin/git-find-rs -d $HOME/code' '$HOME/.local/bin/git-find-repos $HOME/code'
Benchmark 1: /usr/local/bin/git-find-rs $HOME/code
  Time (mean ± σ):     284.2 ms ±   6.8 ms    [User: 45.5 ms, System: 234.2 ms]
  Range (min … max):   274.1 ms … 297.4 ms    10 runs

Benchmark 2: $HOME/.local/bin/git-find-repos $HOME/code
  Time (mean ± σ):     761.3 ms ±  37.6 ms    [User: 203.5 ms, System: 542.5 ms]
  Range (min … max):   733.6 ms … 860.2 ms    10 runs

Summary
  '/usr/local/bin/git-find-rs -d $HOME/code' ran
    2.68 ± 0.15 times faster than '$HOME/.local/bin/git-find-repos $HOME/code'
```

---

## Installation

### From crates.io

```bash
cargo install git-find-rs
```

### From pre-built binaries

Download the latest release for your platform from the [releases page](https://github.com/robgyiv/git-find-rs/releases).

### From source

```bash
# Clone this repo to a local directory, for example `/Users/foo/git-find-rs`
cargo build --release
ln -s $HOME/git-find-rs/target/release/git-find-rs /usr/local/bin/git-find-rs
```

---

## Using in fish shell

Create a function to use `git-find-rs`, for example:

```bash
function repo
    set initial_query $argv
    set code_dir "$HOME/code"
    set dest (git-find-rs "$code_dir" | fzy -q "$initial_query" -l 20) && cd "$dest"
end
```

You can then:

```bash
$ repo my-repo-name
```

This will show a list of git directories. You can type a partial string of the repo name and `fzy` will match this for you. Selecting a directory will `cd` into the directory.

---

## CI/CD

This project uses GitHub Actions for continuous integration and delivery:

- **CI Pipeline**: Runs tests, formatting checks (`cargo fmt`), linting (`cargo clippy`), and documentation checks on every push and PR across multiple platforms (Linux, macOS, Windows) and Rust versions (stable, beta)
- **Security Audits**: Automated dependency vulnerability scanning using `cargo-audit` and `cargo-deny`, running weekly and on dependency changes
- **Automated Releases**: When a version tag is pushed (e.g., `v0.6.0`), the release workflow:
  - Validates the tag matches the `Cargo.toml` version
  - Runs full test suite and linting
  - Publishes to [crates.io](https://crates.io/crates/git-find-rs)
  - Creates a GitHub release with changelog notes
  - Builds and attaches pre-compiled binaries for:
    - Linux (x86_64 glibc/musl, aarch64)
    - macOS (Intel x86_64, Apple Silicon aarch64)
    - Windows (x86_64)
- **Dependabot**: Automatically creates PRs for dependency updates (Rust crates and GitHub Actions)

---

## Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
```

### Creating a Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Commit changes
4. Create and push a version tag:
   ```bash
   git tag v0.6.0
   git push origin v0.6.0
   ```
5. GitHub Actions will automatically publish to crates.io and create a GitHub release

---

## Todo

- Go blazingly fast
- ~~crates.io release~~ ✓
