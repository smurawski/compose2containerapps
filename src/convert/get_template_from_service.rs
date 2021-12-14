use crate::compose::Service;
use crate::containerapps::{ScaleConfiguration, Template};
use anyhow::Result;

use super::get_container_from_service;

pub fn get_template_from_service(service: &Service) -> Result<Template> {
    let template = Template {
        containers: vec![get_container_from_service(service)?],
        revision_suffix: None,
        scale: ScaleConfiguration::default(),
    };
    Ok(template)
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_to_containerapps;
    use crate::convert::tests::{get_sample_cli_args, get_service_from_docker_compose_file};

    #[test]
    fn conversion_sets_template_scale_to_min_1() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

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
