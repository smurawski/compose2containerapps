use crate::compose::{PortMapping, Ports, Protocol, Service};
use crate::containerapps::IngressConfiguration;
use anyhow::Result;
use dialoguer::FuzzySelect;
use log::{debug, trace, warn};

pub fn get_ingress_from_service(service: &Service) -> Result<IngressConfiguration> {
    trace!("Creating the ingress configuration.");
    let mut ingress = IngressConfiguration::default();
    if !service.ports.is_empty() {
        debug!("Service had ports defined.");
        let expanded_ports = expand_service_ports(service);

        debug!("Port collection includes {:?}", &expanded_ports);
        if !expanded_ports.is_empty() {
            let selection = get_selected_port_index(&expanded_ports)?;
            let port = expanded_ports[selection];
            debug!(
                "Setting {} as the publicly accessible port from the container.",
                &port
            );
            ingress.external = true;
            ingress.target_port = Some(port);
        } else {
            warn!("There were no non-TCP ports exposed publicly, but there were publicly exposed ports defined.");
        }
    };
    if !ingress.external && !service.expose.is_empty() {
        debug!("Service had expose defined.");
        let expose_ports = expand_service_expose(service);

        debug!("Port collection includes {:?}", &expose_ports);
        if !expose_ports.is_empty() {
            let selection = get_selected_port_index(&expose_ports)?;
            let port = expose_ports[selection];
            debug!(
                "Setting {} as the internally accessible port from the container.",
                &port
            );
            ingress.target_port = Some(port);
        } else {
            warn!("Expose was defined, but no valid port was defined.");
        }
    };

    Ok(ingress)
}

fn expand_service_expose(service: &Service) -> Vec<u16> {
    let expose_ports_local = service.expose.clone();
    expose_ports_local
        .iter()
        .map(|p| {
            let v = p.value().unwrap().to_owned().parse::<u16>().unwrap();
            debug!("Found exposed port: {}", &v);
            v
        })
        .collect::<Vec<u16>>()
}

fn get_selected_port_index(ports: &[u16]) -> Result<usize> {
    let selection = if ports.len() > 1 {
        FuzzySelect::new()
        .items(ports)
        .with_prompt("ContainerApps can only expose one port.  Please select the target port to expose externally.")
        .interact()?
    } else {
        0
    };
    Ok(selection)
}

fn expand_service_ports(service: &Service) -> Vec<u16> {
    let service_ports = service.ports.clone();
    let port_mappings: Vec<PortMapping> = service_ports
        .iter()
        .map(|pm| pm.value().unwrap().to_owned())
        .collect();

    debug!("Filtering on TCP connections as only HTTP is supported by ContainerApps.");
    port_mappings
        .iter()
        .filter(|p| p.protocol == Protocol::Tcp)
        .map(|p| {
            (match p.container_ports {
                Ports::Port(p) => {
                    debug!("Found an existing port: {}", &p);
                    p..(p + 1)
                }
                Ports::Range(low, high) => {
                    debug!("Found an existing port range: {} - {}", &low, &high);
                    low..(high + 1)
                }
            })
            .collect::<Vec<u16>>()
        })
        .flatten()
        .collect::<Vec<u16>>()
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
