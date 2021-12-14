use crate::compose::{Ports, Service};
use crate::containerapps::IngressConfiguration;
use anyhow::Result;
use log::{debug, trace, warn};

pub fn get_ingress_from_service(service: &Service) -> Result<IngressConfiguration> {
    trace!("Creating the ingress configuration.");
    let mut ingress = IngressConfiguration::default();
    if !service.ports.is_empty() {
        debug!("Service had ports defined.");
        let ports = service.ports.clone();
        let port = ports[0].value()?;
        ingress.external = true;
        ingress.allow_insecure = false;
        ingress.target_port = match port.container_ports {
            Ports::Port(p) => {
                debug!("Found an existing port: {}", &p);
                Some(p)
            }
            Ports::Range(low, high) => {
                debug!("Found an existing port range: {} - {}", &low, &high);
                warn!("Defaulting to use the lower port, but I should really prompt here.");
                Some(low)
            }
        };
    } else if !service.expose.is_empty() {
        debug!("Service had expose defined.");
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
