set shell := [ "bash", "-cu" ]

_default: 
    @just --list --unsorted
    @echo

# Run
[group("run")]
run file:
    @cargo run --release {{file}}

# Build
[group("run")]
build:
    @cargo build 

# Test
[group("run")]
test:
    @cargo test --quiet

# Clean
[group("run")]
clean:
    @cargo clean --quiet

# `fmt`, `clippy` and `test`
[group("vcs")]
ci: 
    @just fmt
    @just clippy
    @just test

# Push
[group("vcs")]
push commit-message: ci
    @git add -A
    @git commit -m "{{commit-message}}"
    @git push origin HEAD

# Format
[group("lint")]
fmt: 
    @cargo fmt 

# Clippy Check
[group("lint")]
clippy:
    @cargo clippy --quiet

# Clippy Fix
[group("lint")]
clippy-fix:
    @cargo clippy --quiet --fix --bin tesuji -p tesuji

