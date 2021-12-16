set dotenv-load := true
set shell := ["pwsh", "-c"]

defaultComposeFile       := "./test/docker-compose.yml"
defaultContainerAppsFile := "skipazure-containerapps.yml"

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

run composeFile=defaultComposeFile: bicep-build
    cargo run -- {{composeFile}}

run-skip-azure composeFile=defaultComposeFile containerappsFile=defaultContainerAppsFile: 
    cargo run -- {{composeFile}} {{containerappsFile}} --skip-azure

run-multiple-service: (run-skip-azure "./test/docker-compose-multiple-service.yml")

run-gamut: run-skip-azure run-multiple-service run-multiple-port

run-multiple-port: (run-skip-azure "./test/docker-compose-multiple-service-multiple-ports.yml" "ports-containerapps.yml")

bicep-build:
    az bicep build --file ./src/support/main.bicep --outdir ./src/support/

cleanup:
    rm *-containerapps.yml
    az group delete --name $env:RESOURCE_GROUP --no-wait -y

demo: && show run show
    Write-Host "Nothing up my sleeve."

show:
    -az group show --name $env:RESOURCE_GROUP

    