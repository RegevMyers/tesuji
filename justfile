set shell := [ "bash", "-c" ]

_default: 
    @just --list

run file:
    @cargo -q run --release {{file}}

test:
    @cargo test

push commit-message:
    @git add -A
    @git commit -m "{{commit-message}}"
    @git push origin HEAD

fmt: 
    @cargo fmt

clippy:
    @cargo clippy

clippy-fix:
    @cargo clippy --fix --bin tesuji -p tesuji

