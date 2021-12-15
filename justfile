set dotenv-load := true
set shell := ["pwsh", "-c"]

defaultComposeFile       := "./test/docker-compose.yml"

export RUST_BACKTRACE := "1"
export RUST_LOG       := "compose2containerapp=trace"

default: lint clippy check test

try: && clippy check
    cargo fmt

lint:
    cargo fmt --all -- --check

clippy:
    cargo clippy -- -D warnings

check:
    cargo check

test:
    cargo test

run composeFile=defaultComposeFile:
    cargo run -- {{composeFile}}

multiple-service: (run "./test/docker-compose-multiple-service.yml")

multiple-port: (run "./test/docker-compose-multiple-service-multiple-ports.yml")

publish:
    $Version = ((cargo run -- -V) -split ' ')[1]
    git tag $Version
    git push origin $Version

