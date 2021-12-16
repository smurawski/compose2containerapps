use crate::azure::*;
use anyhow::Result;
use dialoguer::Input;
use log::{debug, trace};

#[derive(Default)]
pub struct ValidateAzureCommand<'a> {
    subscription_name: Option<&'a str>,
    resource_group: Option<&'a str>,
    location: Option<&'a str>,
    containerapps_environment_name: Option<&'a str>,
    containerapps_environment_resource_id: Option<String>,
}

impl<'a> ValidateAzureCommand<'a> {
    pub fn with_subscription_name(mut self, subscription_name: Option<&'a str>) -> Self {
        self.subscription_name = subscription_name;
        self
    }
    pub fn with_resource_group(mut self, resource_group: Option<&'a str>) -> Self {
        self.resource_group = resource_group;
        self
    }
    pub fn with_location(mut self, location: Option<&'a str>) -> Self {
        self.location = location;
        self
    }

    pub fn with_containerapps_environment_name(
        mut self,
        containerapps_environment_name: Option<&'a str>,
    ) -> Self {
        self.containerapps_environment_name = containerapps_environment_name;
        self
    }

    pub fn with_containerapps_environment_resource_id(
        mut self,
        containerapps_environment_resource_id: Option<&'a str>,
    ) -> Self {
        if let Some(s) = containerapps_environment_resource_id {
            self.containerapps_environment_resource_id = Some(s.to_owned());
        } else {
            self.containerapps_environment_resource_id = None;
        }
        self
    }

    pub fn validate_azure_login(self, skip: bool) -> Result<Self> {
        if !skip {
            trace!("Checking for the az CLI and if we are logged into Azure.");
            let subscription: String = if let Some(s) = self.subscription_name {
                s.to_string()
            } else {
                Input::new()
                    .with_prompt("Please enter the subscription name you would like to use")
                    .interact_text()?
            };
            debug!("Logging in to Azure subscription {}", &subscription);
            set_azure_environment(&subscription)?;
            debug!("Logged in to Azure subscription {}", &subscription);
        }
        Ok(self)
    }

    pub fn retrieve_containerapps_environment(mut self, skip: bool) -> Result<Self> {
        if !skip {
            setup_extensions_and_preview_commands()?;
            if self.resource_group.is_some() && self.containerapps_environment_name.is_some() {
                self.containerapps_environment_resource_id = get_az_containerapp_environment(
                    self.get_resource_group()?,
                    self.get_containerapps_environment_name()?,
                )?;
            }
            if self.containerapps_environment_resource_id.is_none() {
                self.containerapps_environment_resource_id = Some(deploy_containerapps_env(
                    self.get_resource_group()?,
                    self.get_containerapps_environment_name()?,
                    self.get_location()?,
                )?);
            }
        }
        Ok(self)
    }

    pub fn containerapps_environment_id(&self) -> Result<Option<String>> {
        if let Some(v) = self.containerapps_environment_resource_id.clone() {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn get_resource_group(&self) -> Result<&str> {
        let value = self.resource_group.unwrap();
        Ok(value)
    }

    fn get_containerapps_environment_name(&self) -> Result<&str> {
        let value = self.containerapps_environment_name.unwrap();
        Ok(value)
    }

    fn get_location(&self) -> Result<&str> {
        let value = self.location.unwrap();
        Ok(value)
    }
}
