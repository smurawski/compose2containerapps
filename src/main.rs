#[macro_use]
extern crate lazy_static;

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
    pub static ref VALIDATE_AZURE: bool = !get_app_cli(&VERSION)
        .get_matches()
        .is_present("skip_validate_azure");
    pub static ref DEPLOY_AZURE: bool = !get_app_cli(&VERSION)
        .get_matches()
        .is_present("skip_deploy_azure");
}

fn main() -> Result<()> {
    env_logger::init();
    let matches = get_app_cli(&VERSION).get_matches();

    if *VALIDATE_AZURE {
        ValidateAzureCommand::default()
            .with_subscription_name(matches.value_of("subscription_name"))
            .validate_azure_login()?;
        //retrieve_containerapps_environment()
    }
    ConvertComposeCommand::default()
        .with_compose_path(matches.value_of("INPUT").unwrap())
        .with_containerapps_path(matches.value_of("OUTPUT").unwrap())
        .with_resource_group(matches.value_of("resourceGroup"))
        .with_location(matches.value_of("location"))
        .with_kube_environment_id(matches.value_of("kubeEnvironmentId"))
        .convert()?
        .write()?;

    if *DEPLOY_AZURE {};
    Ok(())
}
