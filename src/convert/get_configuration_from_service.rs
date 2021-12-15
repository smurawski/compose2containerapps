use crate::compose::Service;
use crate::containerapps::{Configuration, RevisionMode};
use anyhow::Result;

use super::get_ingress_from_service;
use super::get_secrets_from_service;
use crate::VERBOSE;

pub fn get_configuration_from_service(service: &Service) -> Result<Configuration> {
    if *VERBOSE {
        println!();
        println!("The ContainerApps container configuration is defined https://aka.ms/containerapps/spec#propertiesconfiguration.");
        println!("This configuration includes any secrets for the environment, any container registries, and the ingress.");
        println!("Details on configuring a container registries is located at https://aka.ms/containerapps/containers#container-registries.");
        println!("An environment can include more than one container, like a pod in Kubernetes.");
        println!("activeRevisionsMode is also defined here.  Revisions can be used to control traffic flow.");
        println!("Here we are defaulting to the latest revision, as that's the experience you would have in Docker Compose.");
        println!(
            "You can learn more about revisions at https://aka.ms/containerapps/revisiondetail."
        );
        println!();
    }
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
    use crate::convert::tests::get_converted_containerapps_config;

    #[test]
    fn conversion_defaults_properties_configuration_active_revision_mode_to_single() {
        let new_containerapps_config = get_converted_containerapps_config();

        assert_eq!(
            new_containerapps_config
                .properties
                .configuration
                .active_revisions_mode,
            RevisionMode::Single
        );
    }
}
