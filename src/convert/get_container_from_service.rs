use crate::compose::Service;
use crate::containerapps::{Container, EnvironmentConfiguration};
use crate::VERBOSE;
use anyhow::Result;
use dialoguer::Input;
use log::debug;

pub fn get_container_from_service(service: &Service) -> Result<Container> {
    if *VERBOSE {
        println!();
        println!("The container template includes the container image, an optional name,");
        println!("as well as environment variables. All the options are defined https://aka.ms/containerapps/containers#configuration.");
        println!();
    };
    let mut container = Container::default();
    if let Some(image) = &service.image {
        container.image = image.value()?.to_string();
    }

    if let Some(name) = &service.container_name {
        container.name = Some(name.value()?.to_string());
    }

    if !service.environment.is_empty() {
        debug!("Resolving environment variables for the container configuration.");
        for (key, mut wrapped_value) in service.environment.clone().into_iter() {
            let new_value = match wrapped_value.interpolate() {
                Ok(v) => {
                    debug!("Resolved environment variable for {} to {}", &key, v);
                    Some(v.to_string())
                }
                _ => {
                    debug!("Failed to interpolate the environment variable {}", &key);
                    println!(
                        "Unable to resolve the variable reference for {}",
                        &wrapped_value
                    );
                    let prompt = format!("Please enter a value for {}", key);
                    let value: String = Input::new().with_prompt(prompt).interact_text()?;
                    Some(value)
                }
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
    use crate::convert::tests::*;

    // #[test]
    // fn conversion_prompts_for_undefined_environment_variables() {
    //     let compose_config = get_service_from_docker_compose_file();
    //     compose_config
    //         .environment
    //         .clone()
    //         .into_iter()
    //         .map(|(_, mut env)| println!("{}", env))
    //         .collect::<Vec<_>>();
    // }

    #[test]
    fn conversion_sets_properties_template_containers_image() {
        let new_containerapps_config = get_converted_containerapps_config();

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
        let new_containerapps_config = get_converted_containerapps_config();

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
        let new_containerapps_config = get_converted_containerapps_config();

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
