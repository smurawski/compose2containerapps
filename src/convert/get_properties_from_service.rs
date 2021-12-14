use crate::compose::Service;
use crate::containerapps::Properties;
use anyhow::Result;

use super::get_configuration_from_service;
use super::get_template_from_service;

pub fn get_properties(kube_environment: String, service: &Service) -> Result<Properties> {
    let props = Properties {
        kube_environment_id: kube_environment,
        configuration: get_configuration_from_service(service)?,
        template: get_template_from_service(service)?,
    };
    Ok(props)
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_to_containerapps;
    use crate::convert::tests::{
        get_sample_cli_args, get_service_from_docker_compose_file, read_containerapps_file,
    };

    #[test]
    fn conversion_sets_properties_kube_environment_id_from_cli() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.properties.kube_environment_id,
            reference_containerapps_config
                .properties
                .kube_environment_id
        );
    }
}
