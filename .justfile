set shell := [ "bash", "-cu" ]

@_default: 
    echo
    just --list --unsorted
    echo

# Run
[group("run")]
@run cli="":
    cargo run --release -- {{ cli }}

# Build
[group("run")]
@build:
    cargo build 

# Test
[group("run")]
@test:
    cargo test --quiet

# Clean
[group("run")]
@clean:
    cargo clean --quiet

# Fetch
[group("vcs")]
@fetch:
    git fetch origin

# Pull
[group("vcs")]
@pull: (fetch)
    git pull origin HEAD

# Push
[group("vcs")]
@push msg: (ci)
    git add -A
    git commit -m "{{ msg }}"
    git push origin HEAD

# Format
[group("lint")]
@fmt: 
    cargo fmt 

# Clippy Check
[group("lint")]
@clippy:
    cargo clippy --quiet

# Clippy Fix
[group("lint")]
@clippy-fix:
    cargo clippy --quiet --fix --bin tesuji -p tesuji

# `fmt`, `clippy` and `test`
[group("lint")]
@ci: 
    just fmt
    just clippy
    just test

# Lines of Code
[group("util")]
@loc:
    wc -l $(fdfind -e rs -E mod.rs)
