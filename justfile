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

run-hollan:
    cargo run -- "./test/jeff-hollan-compose.yml" deploy --transport Http2

# run composeFile=defaultComposeFile action=defaultAction
#     cargo run -- {{composeFile}} {{defaultAction}}

# run-deploy:}} ) bicep-build
#     cargo run -- {{composeFile}} deploy

# run-convert composeFile=defaultComposeFile containerappsFile=defaultContainerAppsFile: 
#     cargo run -- {{composeFile}} {{containerappsFile}} convert

# run-multiple-service: (run-convert "./test/docker-compose-multiple-service.yml")

# run-multiple-port: (run-convert "./test/docker-compose-multiple-service-multiple-ports.yml" "ports-containerapps.yml")

# run-gamut: run-convert run-multiple-service run-multiple-port

bicep-build:
    az bicep build --file ./src/support/main.bicep --outdir ./src/support/

cleanup:
    rm *-containerapps.yml
    az group delete --name $env:RESOURCE_GROUP --no-wait -y

# demo: && show run show
#     Write-Host "Nothing up my sleeve."

show:
    -az group show --name $env:RESOURCE_GROUP

    