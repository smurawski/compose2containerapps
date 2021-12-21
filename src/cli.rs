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

arg_enum! {
    pub enum Transport {
        Auto,
        Http,
        Http2
    }
}

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("compose2containerapps")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Converts Docker Compose files to Azure ContainerApps yaml configuration files")
        .subcommand(convert_subcommand())
        .subcommand(deploy_subcommand())
        .subcommand(logs_subcommand())
}

fn convert_subcommand<'a, 'b>() -> App<'a, 'b> {
    let standard_args = standard_args();
    SubCommand::with_name("convert")
        .about("Converts a Docker Compose file into Azure ContainerApps configurations.")
        .args(&standard_args)
        .arg(containerapps_environment_id_arg())
        .arg(resource_group_name_arg())
        .arg(location_arg())
        .arg(transport_arg())
}

fn deploy_subcommand<'a, 'b>() -> App<'a, 'b> {
    let standard_args = standard_args();
    SubCommand::with_name("deploy")
        .about("Deploys a Docker Compose file into Azure ContainerApps")
        .args(&standard_args)
        .arg(containerapps_environment_id_arg())
        .arg(containerapps_environment_name_arg())
        .arg(subscription_name_arg())
        .arg(resource_group_name_arg())
        .arg(location_arg())
        .arg(transport_arg())
}

fn logs_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("logs")
        .about("Retrieves Azure ContainerApps Logs")
        .arg(containerapps_environment_client_id_arg())
        .arg(containerapps_environment_id_arg())
        .arg(containerapps_environment_name_arg())
        .arg(resource_group_name_arg())
        .arg(max_records_arg())
        .arg(containerapps_name_arg())
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
    )
}

fn containerapps_environment_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ContainerAppsEnvironmentName")
        .long("containerapps-environment-name")
        .short("n")
        .help("Resource Name for the ContainerApps environment.")
        .env("CONTAINERAPPS_ENVIRONMENT")
        .takes_value(true)
}

fn containerapps_environment_id_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ContainerAppsEnvironmentId")
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

fn containerapps_environment_client_id_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("log_analytics_client_id")
        .long("log-analtyics-client-id")
        .short("c")
        .help("Resource ID for the ContainerApps environment.")
        .aliases(&["client-id", "workspace-id"])
        .env("LOG_ANALYTICS_WORKSPACE_CLIENT_ID")
        .takes_value(true)
}

fn resource_group_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ResourceGroup")
        .long("resource-group")
        .short("g")
        .help("Resource Group for the ContainerApps environment.")
        .takes_value(true)
        .env("RESOURCE_GROUP")
        .aliases(&["resourcegroup", "resource-group-name", "resourcegroupname"])
}

fn location_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("Location")
        .long("location")
        .short("l")
        .help("Resource group location for the ContainerApps environment.")
        .takes_value(true)
        .possible_values(&Region::variants())
        .env("LOCATION")
}

fn subscription_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("SubscriptionName")
        .long("subscription-name")
        .help("Resource group location for the ContainerApps environment.")
        .takes_value(true)
        .env("AZURE_SUBSCRIPTION_NAME")
}

fn transport_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("Transport")
        .long("transport")
        .help("ContainerApps transport.")
        .takes_value(true)
        .possible_values(&Transport::variants())
}

fn max_records_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("NumberOfResults")
        .long("number-of-results")
        .help("Number of records to return.")
        .takes_value(true)
        .aliases(&["max-records", "max-results"])
        .default_value("100")
}

fn containerapps_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ContainerAppName")
        .long("name")
        .help("Name of the ContainerApp to retrive logs for.")
        .takes_value(true)
}
