use crate::compose::Service;
use crate::containerapps::ContainerAppConfig;
use anyhow::Result;
use std::collections::HashMap;

use super::get_properties;

pub fn convert_to_containerapps(
    service: Service,
    cli_values: &HashMap<&str, String>,
) -> Result<ContainerAppConfig> {
    let config = ContainerAppConfig {
        kind: "containerapp".to_string(),
        name: cli_values["name"].to_owned(),
        resource_group: cli_values["resourceGroup"].to_owned(),
        location: cli_values["location"].to_owned(),
        resource_type: "Microsoft.Web/containerApps".to_string(),
        tags: None,
        properties: get_properties(cli_values["kubeEnvironmentId"].to_owned(), &service)?,
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_to_containerapps;
    use crate::convert::tests::{
        get_sample_cli_args, get_service_from_docker_compose_file, read_containerapps_file,
    };

    #[test]
    fn conversion_sets_name_from_cli() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.name,
            reference_containerapps_config.name
        );
    }

    #[test]
    fn conversion_sets_resource_group_from_cli() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.resource_group,
            reference_containerapps_config.resource_group
        );
    }

    #[test]
    fn conversion_sets_location_from_cli() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.location,
            reference_containerapps_config.location
        );
    }
}
