use clap::{App, Arg};

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("compose2containerapps")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Converts Docker Compose files to Azure ContainerApps yaml configuration files")
        .arg(
            Arg::with_name("INPUT")
                .help("Path to read the Docker Compose yaml configuration file.")
                .index(1)
                .default_value("./docker-compose.yml"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Base file name to write the Azure ContainerApps yaml configuration files.  Output file name will be prefixed with the service name.")
                .index(2)
                .default_value("containerapps.yml"),
        )
        .arg(
            Arg::with_name("kubeEnvironmentName")
                .long("containerapps-environment-name")
                .short("n")
                .help("Resource Name for the ContainerApps environment.")
                .env("CONTAINERAPPS_ENVIRONMENT_NAME")
                .takes_value(true)
                .hidden(true),
        )
        .arg(
            Arg::with_name("kubeEnvironmentId")
                .long("containerapps-environment-id")
                .short("i")
                .help("Resource ID for the ContainerApps environment.")
                .aliases(&[
                    "resource-id",
                    "resourceid",
                    "kubeEnvironmentId",
                    "kube-environment-id",
                ])
                .env("CONTAINERAPPS_ENVIRONMENT_ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("resourceGroup")
                .long("resource-group")
                .short("g")
                .help("Resource Group for the ContainerApps environment.")
                .takes_value(true)
                .env("RESOURCE_GROUP")
                .aliases(&["resourcegroup", "resource-group-name", "resourcegroupname"]),
        )
        .arg(
            Arg::with_name("location")
                .long("location")
                .short("l")
                .help("Resource group location for the ContainerApps environment.")
                .takes_value(true)
                .env("LOCATION"),
        )
        .arg(
            Arg::with_name("subscription_name")
                .long("subscription-name")
                .help("Resource group location for the ContainerApps environment.")
                .takes_value(true)
                .env("AZURE_SUBSCRIPTION_NAME"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Enable verbose output."),
        )
        .arg(
            Arg::with_name("skip_validate_azure")
                .long("skip-validate-azure")
                .short("s")
                .hidden(true)
                .help("Skip logging into Azure via the Azure CLI."),
        )
        .arg(
            Arg::with_name("skip_deploy_azure")
                .long("skip-deploy-azure")
                .short("d")
                .hidden(true)
                .help("Skip deploying the ContainerApps into Azure via the Azure CLI."),
        )
}
