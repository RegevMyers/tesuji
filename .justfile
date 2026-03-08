set shell := [ "bash", "-cu" ]

@_default: 
    just --list --unsorted
    echo

# Run
[group("run")]
@run sgf:
    cargo run --release {{ sgf }}

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

# `fmt`, `clippy` and `test`
[group("vcs")]
@ci: 
    just fmt
    just clippy
    just test

# Push
[group("vcs")]
@push msg: ci
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

