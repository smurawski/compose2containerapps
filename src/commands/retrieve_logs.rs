use crate::azure::*;
use anyhow::{anyhow, Result};
use dialoguer::Input;
use log::{debug, trace};

#[derive(Default)]
pub struct RetrieveLogsCommand {
    log_analytics_workspace_id: String,

}

impl RetrieveLogsCommand {
    fn with_log_analytics_workspace_id(mut self, log_analytics_workspace_id: Option<&str>) {
        if Some(v) = log_analytics_workspace_id {
            self.log_analytics_workspace_id = v.to_owned();
        }
        self
    }
}