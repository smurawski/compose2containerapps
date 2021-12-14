set shell := ["pwsh", "-c"]

default: lint check test

lint:
    cargo fmt --all -- --check
    cargo clippy

check:
    cargo check

test:
    cargo test

