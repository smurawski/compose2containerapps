use crate::compose::{Compose, Ports, Service};
use crate::containerapps::{
    Configuration, Container, ContainerAppConfig, EnvironmentConfiguration, IngressConfiguration,
    Properties, RevisionMode, ScaleConfiguration, SecretsConfiguration, Template,
};
use anyhow::Result;
use std::collections::HashMap;

pub fn convert_to_containerapps(
    compose_file: Compose,
    cli_values: HashMap<&str, String>,
) -> Result<ContainerAppConfig> {
    let config = ContainerAppConfig {
        kind: "containerapp".to_string(),
        name: cli_values["name"].to_owned(),
        resource_group: cli_values["resourceGroup"].to_owned(),
        location: cli_values["location"].to_owned(),
        resource_type: "Microsoft.Web/containerApps".to_string(),
        tags: None,
        properties: get_properties(cli_values["kubeEnvironmentId"].to_owned(), &compose_file)?,
    };

    Ok(config)
}

fn get_properties(kube_environment: String, compose_file: &Compose) -> Result<Properties> {
    let props = Properties {
        kube_environment_id: kube_environment,
        configuration: get_configuration_from_compose(compose_file)?,
        template: get_template_from_compose(compose_file)?,
    };
    Ok(props)
}

fn get_configuration_from_compose(compose_file: &Compose) -> Result<Configuration> {
    let config = Configuration {
        secrets: get_secrets_from_compose(compose_file)?,
        ingress: get_ingress_from_compose(compose_file)?,
        active_revisions_mode: RevisionMode::default(),
    };
    Ok(config)
}

fn get_secrets_from_compose(_compose_file: &Compose) -> Result<Vec<SecretsConfiguration>> {
    let secrets = Vec::new();
    Ok(secrets)
}

fn get_ingress_from_compose(compose_file: &Compose) -> Result<IngressConfiguration> {
    let mut ingress = IngressConfiguration::default();
    let services = get_public_services_from_compose(compose_file);
    let ports = services[0].ports.clone();
    let port = ports[0].value()?;

    ingress.external = true;
    ingress.allow_insecure = false;
    ingress.target_port = match port.container_ports {
        Ports::Port(p) => Some(p),
        Ports::Range(low, _high) => Some(low),
    };

    Ok(ingress)
}

fn get_public_services_from_compose(compose_file: &Compose) -> Vec<Service> {
    let compose = compose_file.clone();
    compose
        .services
        .into_values()
        .filter(|s| !s.ports.is_empty())
        .collect()
}

fn get_template_from_compose(compose_file: &Compose) -> Result<Template> {
    let template = Template {
        containers: get_containers_from_compose(compose_file),
        revision_suffix: None,
        scale: ScaleConfiguration::default(),
    };
    Ok(template)
}

fn get_containers_from_compose(compose_file: &Compose) -> Vec<Container> {
    let mut containers = Vec::new();
    let compose = compose_file.clone();

    for service in compose.services.into_values() {
        if let Ok(container) = get_container_from_service(service) {
            containers.push(container);
        }
    }
    containers
}

fn get_container_from_service(service: Service) -> Result<Container> {
    let mut container = Container::default();
    if let Some(image) = service.image {
        container.image = image.value()?.to_string();
    }

    if let Some(name) = service.container_name {
        container.name = Some(name.value()?.to_string());
    }

    if !service.environment.is_empty() {
        for (key, wrapped_value) in service.environment.into_iter() {
            let new_value = match wrapped_value.value() {
                Ok(v) => Some(v.to_string()),
                _ => None,
            };
            let env = EnvironmentConfiguration {
                name: key,
                value: new_value,
                secret_ref: None,
            };
            container.env.push(env);
        }
    }
    Ok(container)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::containerapps::*;
    use std::fs::File;
    use std::path::Path;

    fn read_docker_compose_file() -> Compose {
        let path = Path::new("test/docker-compose.yml");
        let config: Compose = Compose::read_from_path(&path).unwrap();
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
