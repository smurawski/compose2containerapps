mod convert_to_containerapps;
mod get_configuration_from_compose;
mod get_container_from_service;
mod get_containers_from_compose;
mod get_ingress_from_compose;
mod get_properties;
mod get_public_services_from_compose;
mod get_secrets_from_compose;
mod get_template_from_compose;
mod read_containerapps_file;
mod read_docker_compose_file;

pub use convert_to_containerapps::*;
pub use get_configuration_from_compose::*;
pub use get_container_from_service::*;
pub use get_containers_from_compose::*;
pub use get_ingress_from_compose::*;
pub use get_properties::*;
pub use get_public_services_from_compose::*;
pub use get_secrets_from_compose::*;
pub use get_template_from_compose::*;
pub use read_containerapps_file::*;
pub use read_docker_compose_file::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::*;
    use crate::containerapps::*;
    use std::fs::File;
    use std::path::Path;
    use std::collections::HashMap;

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
