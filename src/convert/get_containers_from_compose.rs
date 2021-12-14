use crate::compose::Compose;
use crate::containerapps::Container;

use super::get_container_from_service;

pub fn get_containers_from_compose(compose_file: &Compose) -> Vec<Container> {
    let mut containers = Vec::new();
    let compose = compose_file.clone();

    for service in compose.services.into_values() {
        if let Ok(container) = get_container_from_service(service) {
            containers.push(container);
        }
    }
    containers
}
