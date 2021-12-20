set dotenv-load := true
set shell := ["pwsh", "-c"]

defaultComposeFile       := "./test/docker-compose.yml"
defaultContainerAppsFile := "containerapps.yml"
defaultAction            := "convert"

default: build-bicep lint clippy check test

try: && check clippy test
    cargo fmt

lint:
    cargo fmt --all -- --check

clippy:
    cargo clippy -- -D warnings

check:
    cargo check

test:
    cargo test

run action *FLAGS: build-bicep
    cargo run -- {{action}} {{FLAGS}}

run-logs: (run "logs")

run-hollan: (run "./test/jeff-hollan-compose.yml" "deploy" "--transport Http2") 

build-bicep:
    az bicep build --file ./src/support/main.bicep --outdir ./src/support/

cleanup:
    rm *-containerapps.yml
    az group delete --name $env:RESOURCE_GROUP --no-wait -y

# demo: && show run show
#     Write-Host "Nothing up my sleeve."

show:
    -az group show --name $env:RESOURCE_GROUP

setup-az-containerapp-cli:
    -az extension add --source https://workerappscliextension.blob.core.windows.net/azure-cli-extension/containerapp-0.2.0-py2.py3-none-any.whl -y
    -az provider register --namespace Microsoft.Web

get-environment:
    -az containerapp env show --resource-group $env:RESOURCE_GROUP --name $env:CONTAINERAPPS_ENVIRONMENT_NAME

    