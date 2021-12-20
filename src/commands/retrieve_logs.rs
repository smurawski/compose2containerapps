use crate::azure::*;
use anyhow::Result;
use dialoguer::Input;
use log::{debug, trace};

#[derive(Default)]
pub struct RetrieveLogsCommand {
    log_analytics_client_id: String,
    resource_group: String,
    name: Option<String>,
    containerapps_environment_name: Option<String>,
    containerapps_environment_resource_id: Option<String>,
}

impl RetrieveLogsCommand {
    pub fn with_log_analytics_client_id(mut self, log_analytics_client_id: Option<&str>) -> Self {
        if let Some(v) = log_analytics_client_id {
            self.log_analytics_client_id = v.to_owned();
        }
        self
    }

    pub fn with_resource_group(mut self, resource_group: Option<&str>) -> Self {
        if let Some(v) = resource_group {
            self.resource_group = v.to_owned();
        }
        self
    }

    pub fn with_name(mut self, name: Option<&str>) -> Self {
        if let Some(v) = name {
            self.name = Some(v.to_owned());
        } else {
            self.name = None;
        }
        self
    }

    pub fn with_containerapps_environment_resource_id(
        mut self,
        containerapps_environment_resource_id: Option<&str>,
    ) -> Self {
        if let Some(s) = containerapps_environment_resource_id {
            self.containerapps_environment_resource_id = Some(s.to_owned());
        } else {
            self.containerapps_environment_resource_id = None;
        }
        self
    }

    pub fn with_containerapps_environment_name(
        mut self,
        containerapps_environment_name: Option<&str>,
    ) -> Self {
        if let Some(s) = containerapps_environment_name {
            self.containerapps_environment_name = Some(s.to_owned());
        } else {
            self.containerapps_environment_name = None;
        }
        self
    }

    pub fn run(self) -> Result<()> {
        trace!("Starting to retrieve logs.");
        let local_self = self.validate_before_run()?;
        let mut result =
            get_az_monitor_logs(&local_self.log_analytics_client_id, &local_self.name)?;
        if result.len() > 0 {
            println!("Timestamp:                Logs:");
        
            result.sort();
            result
                .iter()
                .map(|l| println!("{}: {}", l.time_generated, l.log))
                .for_each(drop);
        }
        else {
            eprintln!("No logs available in the target workspace.");
        }
        Ok(())
    }

    fn validate_before_run(self) -> Result<Self> {
        trace!("Validating that either the resource group and environment name are present");
        trace!("or that the resource id for the ContainerApps environment is present.");
        self.prompt_for_resource_group()?
            .update_workspace_id_from_containerapps_environment()?
            .prompt_for_workspace_client_id()
    }

    fn prompt_for_resource_group(mut self) -> Result<Self> {
        if self.resource_group.is_empty() && self.containerapps_environment_resource_id.is_none() {
            debug!("Resource group is an empty string and a resource id was not provided.");
            self.resource_group = Input::new()
                .with_prompt(
                    "Please supply the Resource Group Name for the Azure ContainerApps instance",
                )
                .interact_text()?;
        }
        Ok(self)
    }

    fn update_workspace_id_from_containerapps_environment(mut self) -> Result<Self> {
        if !self.log_analytics_client_id.is_empty() {
            debug!("Log Analytics Client ID has already been set.");
            return Ok(self);
        };
        if let Some(containerapps_env_name) = &self.containerapps_environment_name {
            debug!(
                "Retrieving the Log Analytics Client ID from {} in {}",
                containerapps_env_name, &self.resource_group
            );
            if let Some(v) = get_az_containerapp_environment_log_workspace_id(
                Some(&self.resource_group),
                Some(containerapps_env_name),
                None,
            )? {
                self.log_analytics_client_id = v;
            }
        } else if let Some(containerapps_resource_id) = &self.containerapps_environment_resource_id
        {
            debug!(
                "Retrieving the Log Analytics Client ID via environment resource id: {}",
                containerapps_resource_id
            );
            if let Some(v) = get_az_containerapp_environment_log_workspace_id(
                None,
                None,
                Some(containerapps_resource_id),
            )? {
                self.log_analytics_client_id = v;
            }
        };
        Ok(self)
    }

    fn prompt_for_workspace_client_id(mut self) -> Result<Self> {
        if self.log_analytics_client_id.is_empty() {
            self.log_analytics_client_id = Input::new()
                .with_prompt(
                    "Please supply the Log Analytics Workspace Client ID for the Azure ContainerApps environment",
                )
                .interact_text()?;
        } else {
            debug!("Log Analytics Client ID has already been set.");
        }
        Ok(self)
    }
}
