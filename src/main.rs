mod cli;
mod compose;
mod containerapps;
mod convert;

use anyhow::Result;
use cli::get_app_cli;
use compose::read_compose_file;
use containerapps::write_to_containerapps_file;
use convert::convert_to_containerapps;
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<()> {
    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );
    let matches = get_app_cli(&version).get_matches();
    let compose_file_path = match matches.value_of("INPUT") {
        Some(p) => Path::new(p),
        None => panic!("Since there is a default value, we should never get here."),
    };
    let containerapps_file_path = match matches.value_of("OUTPUT") {
        Some(p) => p,
        None => panic!("Since there is a default value, we should never get here."),
    };
    let mut map = HashMap::new();
    map.insert("name", matches.value_of("name").unwrap().to_string());
    map.insert(
        "location",
        matches.value_of("location").unwrap().to_string(),
    );
    map.insert(
        "kubeEnvironmentId",
        matches.value_of("kubeEnvironmentId").unwrap().to_string(),
    );
    map.insert(
        "resourceGroup",
        matches.value_of("resourceGroup").unwrap().to_string(),
    );
    let compose_file = read_compose_file(compose_file_path)?;
    for (service_name, service) in compose_file.services {
        let container_file = convert_to_containerapps(service, &map)?;
        let new_path = format!("{}-{}", service_name, containerapps_file_path);
        let per_service_containerapps_file_path = Path::new(&new_path);

        write_to_containerapps_file(per_service_containerapps_file_path, &container_file)?;
    }
    Ok(())
}
