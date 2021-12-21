[![CI](https://github.com/smurawski/compose2containerapps/actions/workflows/build.yml/badge.svg)](https://github.com/smurawski/compose2containerapps/actions/workflows/build.yml)

# Compose2ContainerApps

This is a proof of concept to take Docker Compose files ([following the spec](https://github.com/compose-spec/compose-spec/blob/master/spec.md)) and converting them to YAML files that can be used to deploy [Azure ContainerApps](https://docs.microsoft.com/azure/container-apps?WT.mc_id=containers-52416-stmuraws) services.

## Work To Be Done

There are lots of things that the Compose file can express that are not supported.  We should add some warnings about those configuration elements that are not going to be represented - things like volume mounts, network configurations, etc..

Things to figure out:

- [ ] A compose file can describe multiple containers and their connections.  ContainerApps has two options, multiple containers on the same host (basically a k8s pod) or [separate ContainerApps in the same environment with an HTTP ingress between them](https://docs.microsoft.com/azure/container-apps/connect-apps?tabs=bash&WT.mc_id?containers-52416-stmuraws). Currently, the app creates separate ContainerApps deployments for each service defined.
- [X] A compose file can `expose` multiple ports or ranges (internal ingress) or define multiple `ports` or ranges (external ingress).  ContainerApps exposes one port either internally or externally (exclusive) per ContainerApp.
- [ ] Something else?

## Building The App

This app should build with minimal dependencies.  It's been tested with Rust 1.57.

`cargo build`

## Running The App

The application has three mandatory parameters and two optional ones (that have default values).  There are environment variables that can be provided in place of CLI arguments.

`compose2containerapp --help`

```
compose2containerapps v0.6.1
Steven Murawski <steven.murawski@microsoft.com>
Converts Docker Compose files to Azure ContainerApps yaml configuration files

USAGE:
    compose2containerapp.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    convert    Converts a Docker Compose file into Azure ContainerApps configurations.
    deploy     Deploys a Docker Compose file into Azure ContainerApps
    help       Prints this message or the help of the given subcommand(s)
    logs       Retrieves Azure ContainerApps Logs
```

### Convert

The `convert` subcommand takes a Docker Compose file and translates it into a [YAML configuration file for Azure ContainerApps](https://aka.ms/containerapps/spec).


This YAML file can be deployed with `az containerapp create --name YOURCONTAINERAPPNAME --yaml ./path/to/configuration.yaml`.

`compose2containerapp convert --help`

```
compose2containerapp.exe-convert
Converts a Docker Compose file into Azure ContainerApps configurations.

USAGE:
    compose2containerapp.exe convert [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --containerapps-environment-id <ContainerAppsEnvironmentId>
            Resource ID for the ContainerApps environment. [env:
            CONTAINERAPPS_ENVIRONMENT_ID=/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment]
    -l, --location <Location>
            Resource group location for the ContainerApps environment. [env: LOCATION=eastus]  [possible values: eastus,
            westus, centralus]
    -g, --resource-group <ResourceGroup>
            Resource Group for the ContainerApps environment. [env: RESOURCE_GROUP=hollan_test]

        --transport <Transport>
            ContainerApps transport. [possible values: Auto, Http, Http2]


ARGS:
    <INPUT>     Path to read the Docker Compose yaml configuration file. [default: ./docker-compose.yml]
    <OUTPUT>    Base file name to write the Azure ContainerApps yaml configuration files.  Output file name will be
                prefixed with the service name. [default: containerapps.yml]
```

### Deploy

The `deploy` subcommand will take a Docker Compose file and iterate over the services defined, converting them to supported ContainerApps configurations and deploying them to Azure.

If an existing Azure ContainerApps environment is not present, it will create and configure the dependencies in the targeted resource group.

`compose2containerapp deploy --help`

```
compose2containerapp.exe-deploy
Deploys a Docker Compose file into Azure ContainerApps

USAGE:
    compose2containerapp.exe deploy [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --containerapps-environment-id <ContainerAppsEnvironmentId>
            Resource ID for the ContainerApps environment. [env:
            CONTAINERAPPS_ENVIRONMENT_ID=/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment]
    -n, --containerapps-environment-name <ContainerAppsEnvironmentName>
            Resource Name for the ContainerApps environment. [env: CONTAINERAPPS_ENVIRONMENT=hollantest]

    -l, --location <Location>
            Resource group location for the ContainerApps environment. [env: LOCATION=eastus]  [possible values: eastus,
            westus, centralus]
    -g, --resource-group <ResourceGroup>
            Resource Group for the ContainerApps environment. [env: RESOURCE_GROUP=hollan_test]

        --subscription-name <SubscriptionName>
            Resource group location for the ContainerApps environment. [env: AZURE_SUBSCRIPTION_NAME=ca-stmuraws-demo-
            test]
        --transport <Transport>
            ContainerApps transport. [possible values: Auto, Http, Http2]


ARGS:
    <INPUT>     Path to read the Docker Compose yaml configuration file. [default: ./docker-compose.yml]
    <OUTPUT>    Base file name to write the Azure ContainerApps yaml configuration files.  Output file name will be
                prefixed with the service name. [default: containerapps.yml]
```

### Logs

The `logs` subcommand helps retrieve [logs from Azure Container Apps](https://docs.microsoft.com/azure/container-apps/monitor?tabs=bash&WT.mc_id=containers-52416-stmuraws). The logs are kept in Azure Log Analytics.  This command simplifies what you need to know to retrieve the logs.

`compose2containerapp logs --help`

```
compose2containerapp.exe-logs
Retrieves Azure ContainerApps Logs

USAGE:
    compose2containerapp.exe logs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --name <ContainerAppName>                                          Name of the ContainerApp to retrive logs for.
    -i, --containerapps-environment-id <ContainerAppsEnvironmentId>
            Resource ID for the ContainerApps environment. [env:
            CONTAINERAPPS_ENVIRONMENT_ID=/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment]
    -n, --containerapps-environment-name <ContainerAppsEnvironmentName>
            Resource Name for the ContainerApps environment. [env: CONTAINERAPPS_ENVIRONMENT=hollantest]

        --number-of-results <NumberOfResults>                              Number of records to return. [default: 100]
    -g, --resource-group <ResourceGroup>
            Resource Group for the ContainerApps environment. [env: RESOURCE_GROUP=hollan_test]

    -c, --log-analtyics-client-id <log_analytics_client_id>
            Resource ID for the ContainerApps environment. [env: LOG_ANALYTICS_WORKSPACE_CLIENT_ID=]

```
