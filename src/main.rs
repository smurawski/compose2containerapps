mod cli;
mod compose;
mod containerapps;
mod convert;

use anyhow::Result;
use cli::get_app_cli;
use compose::{read_compose_file, Compose};
use containerapps::write_to_containerapps_file;
use convert::convert_to_containerapps;
// Possible log options are (in order)
// error, warn, info, debug, trace
use log::{debug, trace};
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();

    let map = get_cli_argument_value();
    let compose_file = get_docker_compose_file(&map)?;
    convert_services_to_containerapps(map, compose_file)
}

fn get_cli_argument_value() -> HashMap<&'static str, String> {
    trace!("Starting evaluation of CLI values.");
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let matches = get_app_cli(&version).get_matches();
    let mut map = HashMap::new();
    if let Some(compose_file_path) = matches.value_of("INPUT") {
        debug!("Using input file of {}.", compose_file_path);
        map.insert("composePath", compose_file_path.to_string());
    };
    if let Some(containerapps_file_path) = matches.value_of("OUTPUT") {
        debug!("Using a output base name of {}.", containerapps_file_path);
        map.insert("containerappsPath", containerapps_file_path.to_string());
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
    map
}

fn get_docker_compose_file(map: &HashMap<&'static str, String>) -> Result<Compose> {
    trace!("Starting the conversion from Docker Compose to ContainerApps configuration.");
    let mut compose_file = Compose::default();
    if let Some(path) = map.get("composePath") {
        let compose_file_path = Path::new(&path);
        debug!(
            "Reading the Docker Compose file from {}",
            &compose_file_path.display()
        );
        compose_file = read_compose_file(compose_file_path)?;
    };
    Ok(compose_file)
}

fn convert_services_to_containerapps(
    source_map: HashMap<&'static str, String>,
    compose_file: Compose,
) -> Result<()> {
    let containerapps_file_path = source_map.get("containerappsPath").unwrap();

    let mut map = source_map.clone();
    for (service_name, service) in compose_file.services {
        debug!(
            "Creating a ContainerApps configuration for the {} service.",
            service_name
        );
        let new_path = format!("{}-{}", &service_name, containerapps_file_path);
        let per_service_containerapps_file_path = Path::new(&new_path);
        if map.contains_key("name") {
            map.remove("name");
        }
        map.insert("name", service_name);
        let container_file = convert_to_containerapps(service, &map)?;

        debug!(
            "Writing a ContainerApps configuration to {}.",
            &per_service_containerapps_file_path.display()
        );
        write_to_containerapps_file(per_service_containerapps_file_path, &container_file)?;
    }
    Ok(())
}
