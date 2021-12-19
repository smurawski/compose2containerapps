#![allow(unused_assignments)]

mod az_cli;
mod find_command;

use anyhow::Result;
use duct::{cmd, ReaderHandle};
use find_command::find_command;
use log::{debug, trace};
use std::path::PathBuf;
use std::process::ExitStatus;

pub use az_cli::*;

lazy_static! {
    pub static ref AZ_CLI_PATH: PathBuf = get_az_cli_path().unwrap();
}

const ARM: &str = include_str!("../support/main.json");

#[derive(Clone, Debug)]
pub struct AzCliCommand<'a> {
    name: String,
    path: PathBuf,
    args: Vec<&'a str>,
    stdout: Option<String>,
    stderr: Option<String>,
    exit_status: Option<ExitStatus>,
    verbose: bool,
    show_progress: bool,
}

impl<'a> Default for AzCliCommand<'a> {
    fn default() -> AzCliCommand<'a> {
        AzCliCommand {
            name: "login".to_owned(),
            path: AZ_CLI_PATH.clone(),
            args: Vec::new(),
            stdout: None,
            stderr: None,
            exit_status: None,
            verbose: false,
            show_progress: false,
        }
    }
}

impl<'a> AzCliCommand<'a> {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }
    pub fn with_args(mut self, args: Vec<&'a str>) -> Self {
        self.args = args;
        self
    }
    #[allow(dead_code)]
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    #[allow(dead_code)]
    pub fn with_show_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }
    pub fn get_stdout(&self) -> Option<String> {
        self.stdout.clone()
    }
    #[allow(dead_code)]
    pub fn get_stderr(&self) -> Option<String> {
        self.stderr.clone()
    }
    pub fn success(&self) -> bool {
        if let Some(s) = &self.exit_status {
            s.success()
        } else {
            false
        }
    }
    pub fn run(mut self) -> Result<Self> {
        trace!("Command: {} running", &self.name);
        debug!("\t`az {}`", &self.args.join(" "));
        let output = cmd(&self.path, &self.args)
            .stderr_capture()
            .stdout_capture()
            .unchecked()
            .run()?;
        self.stdout = Some(String::from_utf8(output.stdout)?);
        self.stderr = Some(String::from_utf8(output.stderr)?);
        self.exit_status = Some(output.status);
        debug!("Az CLI command stdout: {:?}", &self.stdout);
        debug!("Az CLI command stderr: {:?}", &self.stderr);
        trace!("Finished with command {}", &self.name);

        Ok(self)
    }
    pub fn stderr_reader(&self) -> Result<ReaderHandle> {
        trace!("Command {} running", &self.name);
        debug!("\t`az {}`", &self.args.join(" "));
        let reader = cmd(&self.path, &self.args).stderr_capture().reader()?;
        trace!("Returning reader handle.");
        Ok(reader)
    }
    #[allow(dead_code)]
    pub fn stdout_reader(&self) -> Result<ReaderHandle> {
        trace!("Command {} running", &self.name);
        debug!("\t`az {}`", &self.args.join(" "));
        let reader = cmd(&self.path, &self.args).stdout_capture().reader()?;
        trace!("Returning reader handle.");
        Ok(reader)
    }
}

fn get_az_cli_path() -> Result<PathBuf> {
    let cmd_name = if cfg!(target_os = "windows") {
        "az.cmd"
    } else {
        "az"
    };
    let cli_path = find_command(cmd_name).expect("Failed to find the Az CLI.  Please install the Az CLI to continue (https://aka.ms/containerapps/install-az-cli) or use --skip-azure to only process the Compose files.");
    Ok(cli_path)
}
