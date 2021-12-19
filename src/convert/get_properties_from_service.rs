use crate::commands::ContainerAppsConfigurationData;
use crate::compose::Service;
use crate::containerapps::Properties;
use crate::VERBOSE;
use anyhow::Result;

use super::get_configuration_from_service;
use super::get_template_from_service;

pub fn get_properties(
    containerapps_configuration_data: &ContainerAppsConfigurationData,
    service_name: &str,
    service: &Service,
) -> Result<Properties> {
    if *VERBOSE {
        println!();
        println!("The properties that for the ContainerApps configuration are defined at https://aka.ms/containerapps/spec#properties.");
        println!("kubeEnvironmentId is the Resource ID for the ContainerApps environment.  More at https://aka.ms/containerapps/environment.");
        println!();
    };
    let props = Properties {
        kube_environment_id: containerapps_configuration_data
            .containerapps_environment_id
            .to_owned(),
        configuration: get_configuration_from_service(containerapps_configuration_data, service)?,
        template: get_template_from_service(service_name, service)?,
    };
    Ok(props)
}

#[cfg(test)]
mod tests {
    use super::super::tests::{get_converted_containerapps_config, read_containerapps_file};

    #[test]
    fn conversion_sets_properties_kube_environment_id_from_cli() {
        let new_containerapps_config = get_converted_containerapps_config();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.properties.kube_environment_id,
            reference_containerapps_config
                .properties
                .kube_environment_id
        );
    }
}
