set shell := ["pwsh", "-c"]

default: lint check test

lint:
    cargo fmt --all -- --check
    cargo clippy

check:
    cargo check

test:
    cargo test

publish:
    $Version = ((cargo run -- -V) -split ' ')[1]
    git tag $Version
    git push origin $Version

