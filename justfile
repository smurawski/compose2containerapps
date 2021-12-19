set dotenv-load := true
set shell := ["pwsh", "-c"]

defaultComposeFile       := "./test/docker-compose.yml"
defaultContainerAppsFile := "skipazure-containerapps.yml"
defaultAction            := "convert"

default: lint clippy check test

try: bicep-build && check clippy test
    cargo fmt

lint:
    cargo fmt --all -- --check

clippy:
    cargo clippy -- -D warnings

check:
    cargo check

test:
    cargo test

run composeFile=defaultComposeFile action=defaultAction:
   cargo run -- {{composeFile}} {{defaultAction}}

run-hollan:
    cargo run -- "./test/jeff-hollan-compose.yml" deploy --transport Http2

bicep-build:
    az bicep build --file ./src/support/main.bicep --outdir ./src/support/

cleanup:
    rm *-containerapps.yml
    az group delete --name $env:RESOURCE_GROUP --no-wait -y

# demo: && show run show
#     Write-Host "Nothing up my sleeve."

show:
    -az group show --name $env:RESOURCE_GROUP

    