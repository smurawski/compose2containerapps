use crate::compose::{Compose, Service};

pub fn get_public_services_from_compose(compose_file: &Compose) -> Vec<Service> {
    let compose = compose_file.clone();
    compose
        .services
        .into_values()
        .filter(|s| !s.ports.is_empty())
        .collect()
}
