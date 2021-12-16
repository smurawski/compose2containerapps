use crate::compose::Service;
use crate::containerapps::{ScaleConfiguration, Template};
use crate::VERBOSE;
use anyhow::Result;

use super::get_container_from_service;

pub fn get_template_from_service(service: &Service) -> Result<Template> {
    if *VERBOSE {
        println!();
        println!("The template defines the container images and scaling configuration.");
        println!("While the internal ingress is HTTP only, you can run multiple container images in a single");
        println!("ContainerApp. They can communicate via localhost and share disk and network resources.");
        println!("The container image details can be found at https://aka.ms/containerapps/containers#configuration.");
        println!("Scaling details can be found at https://aka.ms/containerapps/scaling.");
        println!();
    };
    let template = Template {
        containers: vec![get_container_from_service(service)?],
        revision_suffix: None,
        scale: ScaleConfiguration::default(),
    };
    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::super::tests::get_converted_containerapps_config;

    #[test]
    fn conversion_sets_template_scale_to_min_1() {
        let new_containerapps_config = get_converted_containerapps_config();

        assert_eq!(
            new_containerapps_config
                .properties
                .template
                .scale
                .min_replicas,
            1
        );
    }
}
