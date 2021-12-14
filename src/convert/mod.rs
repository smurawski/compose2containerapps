mod convert_to_containerapps;
mod get_configuration_from_service;
mod get_container_from_service;
mod get_ingress_from_service;
mod get_properties_from_service;
mod get_secrets_from_service;
mod get_template_from_service;

pub use convert_to_containerapps::*;
pub use get_configuration_from_service::*;
pub use get_container_from_service::*;
pub use get_ingress_from_service::*;
pub use get_properties_from_service::*;
pub use get_secrets_from_service::*;
pub use get_template_from_service::*;

#[cfg(test)]
pub mod tests {
    use crate::compose::{Compose, Service};
    use crate::containerapps::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::Path;

    pub fn read_docker_compose_file() -> Compose {
        let path = Path::new("test/docker-compose.yml");
        let config: Compose = Compose::read_from_path(&path).unwrap();
        config
    }

    pub fn read_containerapps_file() -> ContainerAppConfig {
        let file = File::open("test/containerapps-converted.yml").unwrap();
        let config: ContainerAppConfig = serde_yaml::from_reader(file).unwrap();
        config
    }

    pub fn get_service_from_docker_compose_file() -> Service {
        let compose = read_docker_compose_file();
        compose.services.get("ghost").unwrap().to_owned()
    }

    pub fn get_sample_cli_args() -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("name", "mycontainerapp".to_string());
        map.insert("resourceGroup", "myresourcegroup".to_string());
        map.insert("kubeEnvironmentId", "/subscriptions/mysubscription/resourceGroups/myresourcegroup/providers/Microsoft.Web/kubeEnvironments/myenvironment".to_string());
        map.insert("location", "northeurope".to_string());
        map
    }
}
