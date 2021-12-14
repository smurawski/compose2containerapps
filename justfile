set shell := ["pwsh", "-c"]

location          := "eastus"
name              := "mycontainerapp"
resourceGroup     := "myresourcegroup"
kubeEnvironmentId := "/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment"
composeFile       := "./test/docker-compose.yml"


default: lint check test

lint:
    cargo fmt --all -- --check
    cargo clippy

check:
    cargo check

test:
    cargo test

run $RUST_LOG="trace" $RUST_BACKTRACE="1":
    cargo run -- {{composeFile}} -i {{kubeEnvironmentId}} -g {{resourceGroup}} -n {{name}} -l {{location}}

publish:
    $Version = ((cargo run -- -V) -split ' ')[1]
    git tag $Version
    git push origin $Version

