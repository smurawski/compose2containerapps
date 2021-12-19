use crate::compose::{read_compose_file, Compose};
use crate::containerapps::{write_to_containerapps_file, ContainerAppConfig};
use crate::convert::convert_to_containerapps;
use anyhow::Result;
use dialoguer::Input;
use log::{debug, trace};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct ConvertedComposeConfiguration {
    pub configuration: ContainerAppConfig,
    pub resource_group: String,
    pub path: PathBuf,
}

#[derive(Default)]
pub struct ConvertComposeCommand {
    compose_path: PathBuf,
    containerapps_path: PathBuf,
    containerapps_configs: Vec<ConvertedComposeConfiguration>,
    resource_group: Option<String>,
    location: Option<String>,
    containerapps_environment_id: Option<String>,
}

impl ConvertComposeCommand {
    pub fn with_compose_path(mut self, compose_file_path: &str) -> Self {
        if let Ok(p) = PathBuf::from_str(compose_file_path) {
            self.compose_path = p;
        }
        self
    }

    pub fn with_containerapps_path(mut self, containerapps_file_path: &str) -> Self {
        if let Ok(p) = PathBuf::from_str(containerapps_file_path) {
            self.containerapps_path = p;
        }
        self
    }

    pub fn with_resource_group(mut self, resource_group: Option<&str>) -> Self {
        self.resource_group = resource_group.map(|v| v.to_string());
        self
    }

    pub fn with_location(mut self, location: Option<&str>) -> Self {
        self.location = location.map(|v| v.to_string());
        self
    }

    pub fn with_containerapps_environment_id(
        mut self,
        containerapps_environment_id: Option<&str>,
    ) -> Self {
        self.containerapps_environment_id = containerapps_environment_id.map(|v| v.to_string());
        self
    }

    pub fn convert(mut self) -> Result<Self> {
        let compose_file = self.get_docker_compose_file()?;
        self.containerapps_configs = self.convert_services_to_containerapps(compose_file)?;
        Ok(self)
    }

    pub fn write(self) -> Result<Self> {
        for config in self.containerapps_configs.iter() {
            write_to_containerapps_file(&config.path, &config.configuration)?
        }
        Ok(self)
    }

    pub fn get_configurations(&self) -> Vec<ConvertedComposeConfiguration> {
        self.containerapps_configs.to_vec()
    }

    fn get_docker_compose_file(&self) -> Result<Compose> {
        trace!("Starting the conversion from Docker Compose to ContainerApps configuration.");
        debug!(
            "Reading the Docker Compose file from {}",
            &self.compose_path.display()
        );
        read_compose_file(&self.compose_path)
    }

    fn convert_services_to_containerapps(
        &self,
        compose_file: Compose,
    ) -> Result<Vec<ConvertedComposeConfiguration>> {
        let mut containerapps = Vec::new();
        for (service_name, service) in compose_file.services {
            debug!(
                "Creating a ContainerApps configuration for the {} service.",
                service_name
            );
            let new_path = PathBuf::from(format!(
                "{}-{}",
                &service_name,
                &self.containerapps_path.display()
            ));

            let container_file = convert_to_containerapps(
                &service_name,
                service,
                &self.resource_group()?,
                &self.location()?,
                &self.containerapps_environment_id()?,
            )?;

            debug!(
                "Writing a ContainerApps configuration to {}.",
                &new_path.display()
            );

            containerapps.push(ConvertedComposeConfiguration {
                resource_group: self.resource_group()?.to_owned(),
                path: new_path,
                configuration: container_file,
            });
        }
        Ok(containerapps)
    }

    fn resource_group(&self) -> Result<String> {
        let resource_group: String = match &self.resource_group {
            Some(rg) => {
                debug!("ContainerApps resource group set to {}", &rg);
                rg.to_string()
            }
            None => Input::new()
                .with_prompt(
                    "Please supply the Resource Group Name for the Azure ContainerApps instance",
                )
                .interact_text()?,
        };
        Ok(resource_group)
    }

    fn location(&self) -> Result<String> {
        let location: String = match &self.location {
            Some(l) => {
                debug!("ContainerApps location set to {}", &l);
                l.to_string()
            }
            None => Input::new()
                .with_prompt("Please supply an Azure region for the ContainerApps instance")
                .interact_text()?,
        };
        Ok(location)
    }

    fn containerapps_environment_id(&self) -> Result<String> {
        let containerapps_environment_id: String = match &self.containerapps_environment_id {
            Some(i) => {
                debug!("ContainerApps Environment Id set to {}", &i);
                i.to_string()
            }
            None => Input::new()
                .with_prompt(
                    "Please supply the Resource ID for the Azure ContainerApps Environment",
                )
                .interact_text()?,
        };
        Ok(containerapps_environment_id)
    }
}
