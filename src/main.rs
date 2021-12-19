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
    let input = main_matches.value_of("INPUT").unwrap();
    let output = main_matches.value_of("OUTPUT").unwrap();

    if let Some(matches) = main_matches.subcommand_matches("convert") {
        ConvertComposeCommand::default()
            .with_compose_path(input)
            .with_containerapps_path(output)
            .with_resource_group(matches.value_of("resourceGroup"))
            .with_location(matches.value_of("location"))
            .with_containerapps_environment_id(matches.value_of("kubeEnvironmentId"))
            .convert()?
            .write()?;
    };

    if let Some(matches) = main_matches.subcommand_matches("deploy") {
        let containerapps_environment_id = ValidateAzureCommand::default()
            .with_subscription_name(matches.value_of("subscription_name"))
            .with_resource_group(matches.value_of("resourceGroup"))
            .with_containerapps_environment_name(matches.value_of("kubeEnvironmentName"))
            .with_containerapps_environment_resource_id(matches.value_of("kubeEnvironmentId"))
            .with_location(matches.value_of("location"))
            .validate_azure_login()?
            .retrieve_containerapps_environment()?
            .containerapps_environment_id()?;

        let configurations = ConvertComposeCommand::default()
            .with_compose_path(input)
            .with_containerapps_path(output)
            .with_resource_group(matches.value_of("resourceGroup"))
            .with_location(matches.value_of("location"))
            .with_containerapps_environment_id(Some(&containerapps_environment_id))
            .convert()?
            .write()?
            .get_configurations();

        DeployAzureCommand::default()
            .with_configurations(configurations)
            .deploy()?
            .iter()
            .map(|fqdn| println!("Deployed: https://{}", fqdn))
            .for_each(drop);
    }

    Ok(())
}
