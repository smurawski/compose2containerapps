use crate::compose::{Ports, Service};
use crate::containerapps::IngressConfiguration;
use anyhow::Result;

pub fn get_ingress_from_service(service: &Service) -> Result<IngressConfiguration> {
    let mut ingress = IngressConfiguration::default();
    if !service.ports.is_empty() {
        let ports = service.ports.clone();
        let port = ports[0].value()?;
        ingress.external = true;
        ingress.allow_insecure = false;
        ingress.target_port = match port.container_ports {
            Ports::Port(p) => Some(p),
            Ports::Range(low, _high) => Some(low),
        };
    }

    Ok(ingress)
}

#[cfg(test)]
mod tests {
    use crate::containerapps::Transport;
    use crate::convert::convert_to_containerapps;
    use crate::convert::tests::{
        get_sample_cli_args, get_service_from_docker_compose_file, read_containerapps_file,
    };

    #[test]
    fn conversion_defaults_properties_configuration_ingress_transport_to_auto() {
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

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
        let compose_config = get_service_from_docker_compose_file();
        let cli_args = get_sample_cli_args();
        let new_containerapps_config = convert_to_containerapps(compose_config, &cli_args).unwrap();

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
}
