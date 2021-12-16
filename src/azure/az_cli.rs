#![allow(dead_code)]
use super::AzCliCommand;
use super::ARM;
use anyhow::{Error, Result};
use custom_error::custom_error;
use log::{debug, trace};
use regex::Regex;
use serde_json::Value;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

custom_error! {
    pub AzCliError
    Unknown = "unknown error",
    CliMissing = "Unable to find the Azure CLI.",
    InvalidJsonError{source: std::string::FromUtf8Error} = "Failed to convert the output.",
    RegexError{source: regex::Error} = "Regex problem.",
    JsonDeserializationError{source: serde_json::Error} = "JSON error",
    CommandFailure{source: std::io::Error} = "Unable to log in via the Azure CLI",
    NotLoggedIn = "Az CLI is not authenticated.",
    MissingTemplate = "No template available to deploy",
    TemplateFailed = "Deployment did not achieve the desired result.",
}

#[derive(Default, Clone, Debug)]
pub struct AzAccountInfo {
    subscription_name: Option<String>,
    subscription_id: Option<String>,
    tenant_id: Option<String>,
}

pub fn set_azure_environment(subscription: &str) -> Result<()> {
    trace!("Entering set azure environment.");
    println!(
        "Checking to see if the Azure CLI is authenticated and which subscription is default."
    );
    let account = match get_account_info() {
        Ok(a) => a,
        Err(_) => {
            trace!("Failed to get existing login information.  Prompting for new login.");
            login()?;
            println!("Checking for the default subscription.");
            get_account_info()?
        }
    };

    if let Some(account_subscription) = account.subscription_name {
        println!("The default subscription is {}", &account_subscription);

        if !subscription.is_empty() {
            if account_subscription.trim_matches('"') == subscription {
                println!("Subscription already configured correctly.\n");
            } else {
                println!("Setting the target subscription to {}\n", &subscription);
                set_target_subscription(subscription)?;
            }
        }
    }

    Ok(())
}

pub fn setup_extensions_and_preview_commands() -> Result<()> {
    trace!("Enabling the preview extension for az containerapps.");
    let extension_url = "https://workerappscliextension.blob.core.windows.net/azure-cli-extension/containerapp-0.2.0-py2.py3-none-any.whl";
    let _ = AzCliCommand::default()
        .with_name("Enable Preview Extension.")
        .with_args(vec!["extension", "add", "--source", extension_url, "--yes"])
        .run()?;

    trace!("Registering the Microsoft.Web provider.");
    let _ = AzCliCommand::default()
        .with_name("Register Microsoft.Web provider.")
        .with_args(vec!["provider", "register", "--namespace", "Microsoft.Web"])
        .run()?;
    Ok(())
}

pub fn get_az_containerapp_environment(
    resource_group: &str,
    environment_name: &str,
) -> Result<Option<String>> {
    let command = AzCliCommand::default()
        .with_name(format!("Get Az ContainerApps Environment {}", environment_name).as_str())
        .with_args(vec![
            "containerapp",
            "env",
            "show",
            "--resource-group",
            resource_group,
            "--name",
            environment_name,
        ])
        .run()?;

    let stdout = command.get_stdout().unwrap();

    let json: Value = match serde_json::from_str(&stdout) {
        Ok(json_value) => json_value,
        Err(_) => Value::default(),
    };

    if let Some(json_resource_id) = json.get("id") {
        let resource_id = json_resource_id.as_str().unwrap();
        debug!("az containerapp show output: {:?}", resource_id);
        Ok(Some(resource_id.to_owned()))
    } else {
        Ok(None)
    }
}

pub fn deploy_containerapps_env<'a>(
    resource_group: &'a str,
    environment_name: &'a str,
    location: &'a str,
) -> Result<String> {
    create_arm_template()?;

    let resource_group_parameter = format!("rgName={}", resource_group);
    let name_parameter = format!("name={}", environment_name);
    let location_parameter = format!("location={}", location);

    let args = vec![
        "deployment",
        "sub",
        "create",
        "--location",
        location,
        "--template-file",
        "azuredeploy.json",
        "--parameters",
        &resource_group_parameter,
        &name_parameter,
        &location_parameter,
    ];
    let command = AzCliCommand::default()
        .with_name(format!("Deploy ContainerApps Environment {}", environment_name).as_str())
        .with_args(args)
        .run()?;
    let stdout = command.get_stdout().unwrap();

    delete_arm_template()?;

    let v: Value = serde_json::from_str(&stdout)?;
    if let Some(resource_id) = v["properties"]["outputs"]["containerappEnvId"]["value"].as_str() {
        debug!("New environment resource id: {}", resource_id);
        Ok(resource_id.to_owned())
    } else {
        Err(Error::new(AzCliError::TemplateFailed))
    }
}

pub fn deploy_containerapps<'a>(
    name: &'a str,
    resource_group: &'a str,
    yaml_path: &'a Path,
) -> Result<String> {
    trace!("Deploying {} to {}", name, resource_group);
    let args = vec![
        "containerapp",
        "create",
        "--name",
        name,
        "--resource-group",
        resource_group,
        "--yaml",
        yaml_path.to_str().unwrap(),
    ];
    let command = AzCliCommand::default()
        .with_name(format!("Deploy {} to {}", &name, &resource_group).as_str())
        .with_args(args)
        .run()?;
    let stdout = command.get_stdout().unwrap();

    let v: Value = serde_json::from_str(&stdout)?;
    if let Some(fqdn) = v["configuration"]["ingress"]["fqdn"].as_str() {
        debug!("New environment resource id: {}", fqdn);
        Ok(fqdn.to_owned())
    } else {
        Err(Error::new(AzCliError::Unknown))
    }
}

fn create_arm_template() -> Result<()> {
    trace!("Creating ARM template.");
    let mut output = File::create("azuredeploy.json")?;
    write!(output, "{}", ARM)?;
    Ok(())
}
fn delete_arm_template() -> Result<()> {
    trace!("Removing ARM template.");
    remove_file("azuredeploy.json")?;
    Ok(())
}

fn get_account_info() -> Result<AzAccountInfo> {
    let command = AzCliCommand::default()
        .with_name("Show logged in account.")
        .with_args(vec!["account", "show", "--output", "json"])
        .run()?;

    let regex_string = "Please run 'az login' to setup account.";
    let re = Regex::new(regex_string)?;

    let account = AzAccountInfo::default();

    let mut return_value = Ok(account);
    let stdout = &command.get_stdout().unwrap();
    if let Some(_captures) = re.captures(stdout) {
        return_value = Err(Error::new(AzCliError::NotLoggedIn));
    } else {
        let v: Value = serde_json::from_str(stdout)?;

        let current_account = AzAccountInfo {
            subscription_id: Some(v["id"].to_string()),
            subscription_name: Some(v["name"].to_string()),
            tenant_id: Some(v["tenantId"].to_string()),
        };

        return_value = Ok(current_account);
    }

    return_value
}

fn login() -> Result<()> {
    let error_pipe_reader = AzCliCommand::default()
        .with_name("Login")
        .with_args(vec!["login"])
        .stderr_reader()?;

    for line in BufReader::new(error_pipe_reader).lines().flatten() {
        let logged_in_regex = r"^WARNING: (You have logged in\.)";
        let warning_regex = r"^WARNING: (.*)$";
        let warn = Regex::new(warning_regex).expect("Boom");
        let logged_in = Regex::new(logged_in_regex).expect("Boom");

        if let Some(m) = warn.captures(&line) {
            if let Some(m2) = logged_in.captures(&line) {
                println!("{}", &m2[1]);
            } else {
                println!("{}", &m[1]);
            }
        }
    }
    Ok(())
}

fn set_target_subscription(subscription_name: &str) -> Result<()> {
    let command = AzCliCommand::default()
        .with_name("Login")
        .with_args(vec!["account", "set", "--subscription", subscription_name])
        .run()?;

    if command.success() {
        Ok(())
    } else {
        Err(Error::new(AzCliError::Unknown))
    }
}
