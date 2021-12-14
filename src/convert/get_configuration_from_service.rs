use crate::compose::Service;
use crate::containerapps::{Configuration, RevisionMode};
use anyhow::Result;

use super::get_ingress_from_service;
use super::get_secrets_from_service;

pub fn get_configuration_from_service(service: &Service) -> Result<Configuration> {
    let config = Configuration {
        secrets: get_secrets_from_service(service)?,
        ingress: get_ingress_from_service(service)?,
        active_revisions_mode: RevisionMode::default(),
        registries: Vec::new(),
    };
    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::containerapps::RevisionMode;
    use crate::convert::convert_to_containerapps;
    use crate::convert::tests::{get_sample_cli_args, get_service_from_docker_compose_file};

    #[test]
    fn conversion_defaults_properties_configuration_active_revision_mode_to_single() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

        assert_eq!(
            new_containerapps_config
                .properties
                .configuration
                .active_revisions_mode,
            RevisionMode::Single
        );
    }
}
