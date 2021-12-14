use crate::compose::Service;
use crate::containerapps::{Container, EnvironmentConfiguration};
use anyhow::Result;

pub fn get_container_from_service(service: Service) -> Result<Container> {
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
