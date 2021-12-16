use super::ConvertedComposeConfiguration;
use crate::azure::*;
use anyhow::Result;

pub struct DeployAzureCommand {
    configurations: Vec<ConvertedComposeConfiguration>,
}

impl Default for DeployAzureCommand {
    fn default() -> DeployAzureCommand {
        DeployAzureCommand {
            configurations: Vec::new(),
        }
    }
}

impl DeployAzureCommand {
    pub fn with_configurations(
        mut self,
        configurations: Vec<ConvertedComposeConfiguration>,
    ) -> Self {
        self.configurations = configurations;
        self
    }

    pub fn deploy(self, skip: bool) -> Result<Vec<String>> {
        let mut fqdns = Vec::new();
        if !skip {
            for config in self.configurations {
                let name = config.configuration.name;
                let resource_group = config.resource_group;
                let yaml_path = config.path;
                let fqdn = deploy_containerapps(&name, &resource_group, &yaml_path)?;
                fqdns.push(fqdn);
            }
        }
        Ok(fqdns)
    }
}
