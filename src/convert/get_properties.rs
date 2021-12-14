use crate::compose::Compose;
use crate::containerapps::Properties;
use anyhow::Result;

use super::get_configuration_from_compose;
use super::get_template_from_compose;

pub fn get_properties(kube_environment: String, compose_file: &Compose) -> Result<Properties> {
    let props = Properties {
        kube_environment_id: kube_environment,
        configuration: get_configuration_from_compose(compose_file)?,
        template: get_template_from_compose(compose_file)?,
    };
    Ok(props)
}
