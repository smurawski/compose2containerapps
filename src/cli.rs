use clap::{arg_enum, App, Arg, SubCommand};

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Region {
        eastus,
        westus,
        centralus,
    }
}

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    let standard_args = standard_args();
    App::new("compose2containerapps")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Converts Docker Compose files to Azure ContainerApps yaml configuration files")
        .args(&standard_args)
        .subcommand(convert_subcommand())
        .subcommand(deploy_subcommand())
}

fn deploy_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("deploy")
        .about("Deploys a Docker Compose file into Azure ContainerApps")
        .arg(containerapps_environment_id_arg())
        .arg(containerapps_environment_name_arg())
        .arg(resource_group_name_arg())
        .arg(location_arg())
        .arg(subscription_name_arg())
}

fn convert_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("convert")
        .about("Converts a Docker Compose file into Azure ContainerApps configurations.")
        .arg(containerapps_environment_name_arg())
        .arg(resource_group_name_arg())
        .arg(location_arg())
}

fn standard_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec!(
        Arg::with_name("INPUT")
            .help("Path to read the Docker Compose yaml configuration file.")
            .index(1)
            .default_value("./docker-compose.yml"),

        Arg::with_name("OUTPUT")
                .help("Base file name to write the Azure ContainerApps yaml configuration files.  Output file name will be prefixed with the service name.")
                .index(2)
                .default_value("containerapps.yml"),

        Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Enable verbose output."),
    )
}

fn containerapps_environment_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("kubeEnvironmentName")
        .long("containerapps-environment-name")
        .short("n")
        .help("Resource Name for the ContainerApps environment.")
        .env("CONTAINERAPPS_ENVIRONMENT_NAME")
        .takes_value(true)
        .hidden(true)
}

fn containerapps_environment_id_arg<'a, 'b>() -> Arg<'a, 'b> {
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
        .takes_value(true)
}

fn resource_group_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("resourceGroup")
        .long("resource-group")
        .short("g")
        .help("Resource Group for the ContainerApps environment.")
        .takes_value(true)
        .env("RESOURCE_GROUP")
        .aliases(&["resourcegroup", "resource-group-name", "resourcegroupname"])
}

fn location_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("location")
        .long("location")
        .short("l")
        .help("Resource group location for the ContainerApps environment.")
        .takes_value(true)
        .possible_values(&Region::variants())
        .env("LOCATION")
}

fn subscription_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("subscription_name")
        .long("subscription-name")
        .help("Resource group location for the ContainerApps environment.")
        .takes_value(true)
        .env("AZURE_SUBSCRIPTION_NAME")
}
