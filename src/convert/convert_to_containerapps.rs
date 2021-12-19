use crate::commands::ContainerAppsConfigurationData;
use crate::compose::Service;
use crate::containerapps::ContainerAppConfig;
use crate::VERBOSE;
use anyhow::Result;

use super::get_properties;

pub fn convert_to_containerapps(
    service_name: &str,
    service: Service,
    containerapps_configuration_data: ContainerAppsConfigurationData,
) -> Result<ContainerAppConfig> {
    if *VERBOSE {
        println!();
        println!(
            "The ContainerApps configuration file is documented at https://aka.ms/containerapps/spec."
        );
        println!();
    };
    let config = ContainerAppConfig {
        kind: Some("containerapp".to_string()),
        api_version: None,
        name: service_name.to_owned(),
        resource_group: Some(containerapps_configuration_data.resource_group.to_owned()),
        location: containerapps_configuration_data.location.to_owned(),
        resource_type: "Microsoft.Web/containerApps".to_string(),
        tags: None,
        properties: get_properties(&containerapps_configuration_data, service_name, &service)?,
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::super::tests::{get_converted_containerapps_config, read_containerapps_file};

    #[test]
    fn conversion_sets_name_from_cli() {
        let new_containerapps_config = get_converted_containerapps_config();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.name,
            reference_containerapps_config.name
        );
    }

    #[test]
    fn conversion_sets_resource_group_from_cli() {
        let new_containerapps_config = get_converted_containerapps_config();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.resource_group,
            reference_containerapps_config.resource_group
        );
    }

    #[test]
    fn conversion_sets_location_from_cli() {
        let new_containerapps_config = get_converted_containerapps_config();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.location,
            reference_containerapps_config.location
        );
    }
}
