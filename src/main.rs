#[macro_use]
extern crate lazy_static;
extern crate clap;

mod azure;
mod cli;
mod commands;
mod compose;
mod containerapps;
mod convert;

use anyhow::Result;
use cli::get_app_cli;
use commands::*;

// Set the RUST_LOG environment variable to control the log output
// Possible log options are (in order). Log levels are cumulative.
// Setting warn includes error.  Info includes warn and error.
// error, warn, info, debug, trace

lazy_static! {
    pub static ref VERSION: String = format!("v{}", env!("CARGO_PKG_VERSION"));
    pub static ref VERBOSE: bool = get_app_cli(&VERSION).get_matches().is_present("verbose");
}

fn main() -> Result<()> {
    env_logger::init();
    let main_matches = get_app_cli(&VERSION).get_matches();

    if let Some(matches) = main_matches.subcommand_matches("convert") {
        ConvertComposeCommand::default()
            .with_compose_path(matches.value_of("INPUT").unwrap())
            .with_containerapps_path(matches.value_of("OUTPUT").unwrap())
            .with_resource_group(matches.value_of("ResourceGroup"))
            .with_location(matches.value_of("Location"))
            .with_containerapps_environment_id(matches.value_of("ContainerAppsEnvironmentId"))
            .with_transport(matches.value_of("Transport"))
            .convert()?
            .write()?;
    };

    if let Some(matches) = main_matches.subcommand_matches("deploy") {
        let containerapps_environment_id = ValidateAzureCommand::default()
            .with_subscription_name(matches.value_of("SubscriptionName"))
            .with_resource_group(matches.value_of("ResourceGroup"))
            .with_containerapps_environment_name(matches.value_of("ContainerAppsEnvironmentName"))
            .with_containerapps_environment_resource_id(
                matches.value_of("ContainerAppsEnvironmentId"),
            )
            .with_location(matches.value_of("Location"))
            .validate_azure_login()?
            .retrieve_containerapps_environment()?
            .containerapps_environment_id()?;

        ConvertComposeCommand::default()
            .with_compose_path(matches.value_of("INPUT").unwrap())
            .with_containerapps_path(matches.value_of("OUTPUT").unwrap())
            .with_resource_group(matches.value_of("ResourceGroup"))
            .with_location(matches.value_of("Location"))
            .with_containerapps_environment_id(Some(&containerapps_environment_id))
            .with_transport(matches.value_of("Transport"))
            .with_deploy_azure(true)
            .convert()?
            .get_configurations()
            .iter()
            .map(|configuration| {
                println!("Deployed: https://{}", &configuration.url.as_ref().unwrap())
            })
            .for_each(drop);
    }

    if let Some(matches) = main_matches.subcommand_matches("logs") {
        RetrieveLogsCommand::default()
            .with_log_analytics_client_id(matches.value_of("log_analytics_client_id"))
            .with_resource_group(matches.value_of("ResourceGroup"))
            .with_name(matches.value_of("ContainerAppName"))
            .with_containerapps_environment_name(matches.value_of("ContainerAppsEnvironmentName"))
            .with_containerapps_environment_resource_id(
                matches.value_of("ContainerAppsEnvironmentId"),
            )
            .with_max_results(matches.value_of("NumberOfResults"))
            .run()?;
    }

    Ok(())
}
