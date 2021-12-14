use crate::compose::{Compose, Ports};
use crate::containerapps::IngressConfiguration;
use anyhow::Result;

use super::get_public_services_from_compose;

pub fn get_ingress_from_compose(compose_file: &Compose) -> Result<IngressConfiguration> {
    let mut ingress = IngressConfiguration::default();
    let services = get_public_services_from_compose(compose_file);
    let ports = services[0].ports.clone();
    let port = ports[0].value()?;

    ingress.external = true;
    ingress.allow_insecure = false;
    ingress.target_port = match port.container_ports {
        Ports::Port(p) => Some(p),
        Ports::Range(low, _high) => Some(low),
    };

    Ok(ingress)
}
