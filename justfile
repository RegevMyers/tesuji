default: 
    @just --list

run file:
    @cargo -q run --release {{file}}

test:
    @cargo test

fmt: 
    @cargo fmt

clippy:
    @cargo clippy

clippy-fix:
    @cargo clippy --fix --bin tesuji -p tesuji

