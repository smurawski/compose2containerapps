set shell := ["pwsh", "-c"]

default: _build-bicep lint clippy check test

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

run action *FLAGS: _build-bicep
    cargo run -- {{action}} {{FLAGS}}

logs *FLAGS: (run "logs" FLAGS)

convert *FLAGS: (run "convert" FLAGS)

@convert-default:
    just --dotenv-path test/default_convert/convert.env convert test/default_convert/docker-compose.yml

@convert-hollan:
    just --dotenv-path test/jeff_hollan_sample/convert.env convert test/jeff_hollan_sample/docker-compose.yml hollan.yml

@convert-multiple:
    just --dotenv-path test/multiple_service_convert/.env convert test/multiple_service_convert/docker-compose.yml multiple-aca.yml

@convert-multiple-port:
    just --dotenv-path test/multiple_services_and_ports_convert/.env convert test/multiple_services_and_ports_convert/docker-compose.yml multiple-ports-aca.yml

@demo-convert: convert-default convert-hollan convert-multiple convert-multiple-port
    ls *.yml

_build-bicep:
    az bicep build --file ./src/support/main.bicep --outdir ./src/support/


setup-az-containerapp-cli:
    -az extension add --source https://workerappscliextension.blob.core.windows.net/azure-cli-extension/containerapp-0.2.0-py2.py3-none-any.whl -y
    -az provider register --namespace Microsoft.Web

help: 
    cargo run -- --help
    cargo run -- convert --help
    cargo run -- deploy --help
    cargo run -- logs --help
    