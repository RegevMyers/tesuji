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
    git pull

timestamp := shell("date -u")

# Push
[group("vcs")]
@push +msg=timestamp: (ci)
    git add -A
    - git commit -m "{{ msg }}"
    git push -u origin HEAD

# Create Branch
[group("vcs")]
@branch name: (fetch)
    git switch -c {{ name }} origin/dev

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

# Run `fmt`, `clippy` and `test`
[group("lint")]
@ci: 
    just fmt
    just clippy
    just test

# Lines of Code
[group("util")]
@loc:
    fdfind -e rs -E mod.rs --strip-cwd-prefix -0 | xargs -0 wc -l | sort -n -r | sed -E $'1s|(.*)|\033[1m\\1\033[0m|'

# Clear
[group("util")]
@clear:
    clear

