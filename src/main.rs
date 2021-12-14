mod cli;
mod compose;
mod containerapps;
mod convert;

use anyhow::Result;
use cli::get_app_cli;
use compose::read_compose_file;
use containerapps::write_to_containerapps_file;
use convert::convert_to_containerapps;
// Possible log options are (in order)
// error, warn, info, debug, trace
use log::{debug, trace};
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();
    trace!("Starting evaluation of CLI values.");
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let matches = get_app_cli(&version).get_matches();
    let compose_file_path = match matches.value_of("INPUT") {
        Some(p) => {
            debug!("Using input file of {}.", p);
            Path::new(p)
        }
        None => panic!("Since there is a default value, we should never get here."),
    };
    let containerapps_file_path = match matches.value_of("OUTPUT") {
        Some(p) => {
            debug!("Using a output base name of {}.", p);
            p
        }
        None => panic!("Since there is a default value, we should never get here."),
    };
    let mut map = HashMap::new();
    if let Some(name) = matches.value_of("name") {
        debug!("ContainerApps name set to {}.", name);
        map.insert("name", name.to_string());
    };
    if let Some(location) = matches.value_of("location") {
        debug!("ContainerApps location set to {}.", location);
        map.insert("location", location.to_string());
    };
    if let Some(kube_environment_id) = matches.value_of("kubeEnvironmentId") {
        debug!(
            "ContainerApps Environment resource id set to {}",
            kube_environment_id
        );
        map.insert("kubeEnvironmentId", kube_environment_id.to_string());
    };
    if let Some(resource_group) = matches.value_of("resourceGroup") {
        debug!("ContainerApps resource group set to {}", resource_group);
        map.insert("resourceGroup", resource_group.to_string());
    };

    trace!("Finished evaluation of CLI values.");
    trace!("Starting the conversion from Docker Compose to ContainerApps configuration.");

    debug!(
        "Reading the Docker Compose file from {}",
        &compose_file_path.display()
    );
    let compose_file = read_compose_file(compose_file_path)?;
    for (service_name, service) in compose_file.services {
        debug!(
            "Creating a ContainerApps configuration for the {} service.",
            service_name
        );
        let container_file = convert_to_containerapps(service, &map)?;
        let new_path = format!("{}-{}", service_name, containerapps_file_path);
        let per_service_containerapps_file_path = Path::new(&new_path);

        debug!(
            "Writing a ContainerApps configuration to {}.",
            &per_service_containerapps_file_path.display()
        );
        write_to_containerapps_file(per_service_containerapps_file_path, &container_file)?;
    }
    Ok(())
}
