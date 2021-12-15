use crate::azure::set_azure_environment;
use anyhow::Result;
use dialoguer::Input;
use log::{debug, trace};

#[derive(Default)]
pub struct ValidateAzureCommand<'a> {
    subscription_name: Option<&'a str>,
}

impl<'a> ValidateAzureCommand<'a> {
    pub fn with_subscription_name(mut self, subscription_name: Option<&'a str>) -> Self {
        self.subscription_name = subscription_name;
        self
    }

    pub fn validate_azure_login(self) -> Result<Self> {
        trace!("Checking for the az CLI and if we are logged into Azure.");
        let subscription: String = if let Some(s) = self.subscription_name {
            s.to_string()
        } else {
            Input::new()
                .with_prompt("Please enter the subscription name you would like to use.")
                .interact_text()?
        };
        debug!("Logging in to Azure subscription {}", &subscription);
        set_azure_environment(&subscription)?;
        debug!("Logged in to Azure subscription {}", &subscription);
        Ok(self)
    }
}

// pub fn retrieve_containerapps_environment() {}

// }
