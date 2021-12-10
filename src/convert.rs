use crate::compose::Compose;
use crate::containerapps::{Container, ContainerAppConfig, EnvironmentConfiguration};
use anyhow::Result;
use std::collections::HashMap;

pub fn convert_to_containerapps(
    compose_file: Compose,
    cli_values: HashMap<&str, String>,
) -> Result<ContainerAppConfig> {
    let mut config = ContainerAppConfig::default();

    config.name = cli_values["name"].to_owned();
    config.resource_group = cli_values["resourceGroup"].to_owned();
    config.location = cli_values["location"].to_owned();

    config.properties.kube_environment_id = cli_values["kubeEnvironmentId"].to_owned();

    let first_service_name = compose_file.services.keys().next().unwrap();
    config.properties.configuration.ingress.target_port = Some(
        compose_file.services[first_service_name]
            .expose
            .first()
            .unwrap()
            .parse()?,
    );

    let mut containers = Vec::new();
    for (_service_name, service_configuration) in compose_file.services.iter() {
        let mut container = Container::default();
        container.image = service_configuration.image.to_owned();
        if service_configuration.container_name.is_some() {
            let containername = service_configuration.container_name.clone().unwrap();
            container.name = Some(containername);
        }
        container.env = service_configuration
            .environment
            .iter()
            .map(|env| {
                let mut split = env.split("=");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                EnvironmentConfiguration {
                    name: key.to_string(),
                    value: Some(value.to_string()),
                    secret_ref: None,
                }
            })
            .collect();
        containers.push(container);
    }
    config.properties.template.containers = containers;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::containerapps::*;
    use std::fs::File;

    fn read_docker_compose_file() -> Compose {
        let file = File::open("test/docker-compose.yml").unwrap();
        let config: Compose = serde_yaml::from_reader(file).unwrap();
        config
    }

    fn read_containerapps_file() -> ContainerAppConfig {
        let file = File::open("test/containerapps-converted.yml").unwrap();
        let config: ContainerAppConfig = serde_yaml::from_reader(file).unwrap();
        config
    }

    fn get_sample_cli_args() -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("name", "mycontainerapp".to_string());
        map.insert("resourceGroup", "myresourcegroup".to_string());
        map.insert("kubeEnvironmentId", "/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment".to_string());
        map.insert("location", "northeurope".to_string());
        map
    }

    #[test]
    fn conversion_sets_name_from_cli() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.name,
            reference_containerapps_config.name
        );
    }

    #[test]
    fn conversion_sets_resource_group_from_cli() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.resource_group,
            reference_containerapps_config.resource_group
        );
    }

    #[test]
    fn conversion_sets_location_from_cli() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.location,
            reference_containerapps_config.location
        );
    }

    #[test]
    fn conversion_sets_properties_kube_environment_id_from_cli() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config.properties.kube_environment_id,
            reference_containerapps_config
                .properties
                .kube_environment_id
        );
    }

    #[test]
    fn conversion_defaults_properties_configuration_active_revision_mode_to_single() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        assert_eq!(
            new_containerapps_config
                .properties
                .configuration
                .active_revisions_mode,
            RevisionMode::Single
        );
    }

    #[test]
    fn conversion_defaults_properties_configuration_ingress_transport_to_auto() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        assert_eq!(
            new_containerapps_config
                .properties
                .configuration
                .ingress
                .transport,
            Transport::Auto
        );
    }

    #[test]
    fn conversion_sets_properties_configuration_ingress_target_port() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        assert_eq!(
            new_containerapps_config
                .properties
                .configuration
                .ingress
                .target_port,
            reference_containerapps_config
                .properties
                .configuration
                .ingress
                .target_port
        );
    }

    #[test]
    fn conversion_sets_properties_template_containers_image() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        let new_ghost_container = new_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap();
        let reference_ghost_container = reference_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap();

        assert_eq!(new_ghost_container.image, reference_ghost_container.image);
    }

    #[test]
    fn conversion_sets_properties_template_containers_name() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        let new_ghost_container = new_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap();
        let reference_ghost_container = reference_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap();
        assert_eq!(
            new_ghost_container.name.clone().unwrap(),
            reference_ghost_container.name.clone().unwrap()
        );
    }

    #[test]
    fn conversion_sets_properties_template_containers_environment() {
        let compose_config = read_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, cli_args).unwrap();

        let reference_containerapps_config = read_containerapps_file();
        let new_ghost_container_env = new_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap()
            .env
            .first()
            .unwrap();
        let reference_ghost_container_env = reference_containerapps_config
            .properties
            .template
            .containers
            .first()
            .unwrap()
            .env
            .first()
            .unwrap();

        assert_eq!(
            new_ghost_container_env.name,
            reference_ghost_container_env.name
        );
        assert_eq!(
            new_ghost_container_env.value.clone().unwrap(),
            reference_ghost_container_env.value.clone().unwrap()
        );
    }
}
