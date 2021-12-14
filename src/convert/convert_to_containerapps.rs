use crate::compose::Compose;
use crate::containerapps::ContainerAppConfig;
use anyhow::Result;
use std::collections::HashMap;

use super::get_properties;

pub fn convert_to_containerapps(
    compose_file: Compose,
    cli_values: HashMap<&str, String>,
) -> Result<ContainerAppConfig> {
    let config = ContainerAppConfig {
        kind: "containerapp".to_string(),
        name: cli_values["name"].to_owned(),
        resource_group: cli_values["resourceGroup"].to_owned(),
        location: cli_values["location"].to_owned(),
        resource_type: "Microsoft.Web/containerApps".to_string(),
        tags: None,
        properties: get_properties(cli_values["kubeEnvironmentId"].to_owned(), &compose_file)?,
    };

    Ok(config)
}
