#![allow(dead_code)]

use super::ARM;
use custom_error::custom_error;
use duct::cmd;
use log::{debug, trace};
use regex::Regex;
use serde_json::Value;
use std::env;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Output;

//use uuid::Uuid;

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

fn get_az_cli_path() -> Result<PathBuf, AzCliError> {
    let cmd_name = if cfg!(target_os = "windows") {
        "az.cmd"
    } else {
        "az"
    };
    if let Some(cli_path) = find_command(cmd_name) {
        Ok(cli_path)
    } else {
        Err(AzCliError::CliMissing)
    }
}

#[derive(Default, Clone, Debug)]
pub struct AzAccountInfo {
    subscription_name: Option<String>,
    subscription_id: Option<String>,
    tenant_id: Option<String>,
}

pub fn set_azure_environment(subscription: &str) -> Result<(), AzCliError> {
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

pub fn setup_extensions_and_preview_commands() -> Result<(), AzCliError> {
    trace!("Enabling the preview extension for az containerapps.");
    let extension_url = "https://workerappscliextension.blob.core.windows.net/azure-cli-extension/containerapp-0.2.0-py2.py3-none-any.whl";
    let _ =
        run_az_command_with_output(vec!["extension", "add", "--source", extension_url, "--yes"])?;
    trace!("Registering the Microsoft.Web provider.");
    let _ =
        run_az_command_with_output(vec!["provider", "register", "--namespace", "Microsoft.Web"])?;
    Ok(())
}

pub fn get_az_containerapp_environment(
    resource_group: &str,
    environment_name: &str,
) -> Result<Option<String>, AzCliError> {
    let output = run_az_command_with_output(vec![
        "containerapp",
        "env",
        "show",
        "--resource-group",
        resource_group,
        "--name",
        environment_name,
    ])?;
    let stdout = String::from_utf8(output.stdout)?;

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

pub fn deploy_containerapps_env(
    resource_group: &str,
    environment_name: &str,
    location: &str,
) -> Result<String, AzCliError> {
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
    let output = run_az_command_with_output(args)?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    debug!("Template run stdout: {}", &stdout);
    debug!("Template run stderr: {}", &stderr);

    delete_arm_template()?;

    let v: Value = serde_json::from_str(&stdout)?;
    if let Some(resource_id) = v["properties"]["outputs"]["containerappEnvId"]["value"].as_str() {
        debug!("New environment resource id: {}", resource_id);
        Ok(resource_id.to_owned())
    } else {
        Err(AzCliError::TemplateFailed)
    }
}

pub fn deploy_containerapps(
    name: &str,
    resource_group: &str,
    yaml_path: &Path,
) -> Result<String, AzCliError> {
    debug!("Deploying {} to {}", name, resource_group);
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
    let output = run_az_command_with_output(args)?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    debug!("ContainerApps deployment run stdout: {}", &stdout);
    debug!("ContainerApps deployment run stderr: {}", &stderr);
    let v: Value = serde_json::from_str(&stdout)?;
    if let Some(fqdn) = v["configuration"]["ingress"]["fqdn"].as_str() {
        debug!("New environment resource id: {}", fqdn);
        Ok(fqdn.to_owned())
    } else {
        Err(AzCliError::Unknown)
    }
}

fn create_arm_template() -> Result<(), AzCliError> {
    trace!("Creating ARM template.");
    let mut output = File::create("azuredeploy.json")?;
    write!(output, "{}", ARM)?;
    Ok(())
}
fn delete_arm_template() -> Result<(), AzCliError> {
    trace!("Removing ARM template.");
    remove_file("azuredeploy.json")?;
    Ok(())
}

fn get_account_info() -> Result<AzAccountInfo, AzCliError> {
    let args = vec!["account", "show", "--output", "json"];

    let output = run_az_command_with_output(args)?;
    let stdout = String::from_utf8(output.stdout)?;

    let regex_string = "Please run 'az login' to setup account.";
    let re = Regex::new(regex_string)?;

    let account = AzAccountInfo::default();

    let mut _return_value = Ok(account);

    if let Some(_captures) = re.captures(&stdout) {
        _return_value = Err(AzCliError::NotLoggedIn);
    } else {
        let v: Value = serde_json::from_str(&stdout)?;

        let current_account = AzAccountInfo {
            subscription_id: Some(v["id"].to_string()),
            subscription_name: Some(v["name"].to_string()),
            tenant_id: Some(v["tenantId"].to_string()),
        };

        _return_value = Ok(current_account);
    }

    _return_value
}

fn login() -> Result<(), AzCliError> {
    let az_cli_path = get_az_cli_path()?;

    let args = vec!["login"];

    let child = cmd(az_cli_path, &args);
    let error_pipe_reader = child.stderr_capture().reader()?;

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

fn set_target_subscription(subscription_name: &str) -> Result<AzAccountInfo, AzCliError> {
    let mut account = get_account_info()?;

    if account.subscription_name != Some(String::from(subscription_name)) {
        let args = vec!["account", "set", "--subscription", subscription_name];
        let _output = run_az_command_with_output(args)?;
        account = get_account_info()?;
    }

    Ok(account)
}

// pub fn run_cli_command(command: &Command) -> Result<Output, AzCliError> {
//     create_resource_group(command)?;

//     let mut args: Vec<String> = Vec::new();
//     let cli_command = command.cli();
//     args.push(cli_command.subcommand());

//     if cli_command.parameters.is_some() {
//         let parameters = cli_command.parameters();
//         args.extend(parameters);
//     }

//     let p: Vec<&str> = args.iter().map(|s| &**s).collect();
//     run_az_command_with_output(p)
// }

// pub fn deploy_template(command: &Command) -> Result<Output, AzCliError> {
//     create_resource_group(command)?;

//     let local_template = command.clone().template();
//     if local_template.path.is_some() {
//         deploy_template_from_file(command)
//     } else if local_template.url.is_some() {
//         deploy_template_from_url(command)
//     } else {
//         Err(AzCliError::MissingTemplate)
//     }
// }

// fn deploy_template_from_file(command: &Command) -> Result<Output, AzCliError> {
//     let local_command = command.clone();
//     let template = local_command.template();
//     let rg = local_command.resource_group.unwrap();
//     let path = template.path();
//     let parameters = template.parameters();
//     let deployment_name = format!("{}", Uuid::new_v4());
//     let mut args = vec![
//         "group",
//         "deployment",
//         "create",
//         "--name",
//         &deployment_name,
//         "--resource-group",
//         &rg,
//         "--template-file",
//         &path,
//     ];

//     if template.parameters.is_some() {
//         args.push("--parameters");

//         let p: Vec<&str> = parameters.iter().map(|s| &**s).collect();
//         args.extend(p);
//     }

//     run_az_command_with_output(args)
// }

// fn deploy_template_from_url(command: &Command) -> Result<Output, AzCliError> {
//     let local_command = command.clone();
//     let template = local_command.template.unwrap();
//     let rg = local_command.resource_group.unwrap();
//     let parameters = template.parameters();
//     let url = template.url();
//     let deployment_name = format!("{}", Uuid::new_v4());
//     let mut args = vec![
//         "group",
//         "deployment",
//         "create",
//         "--name",
//         &deployment_name,
//         "--resource-group",
//         &rg,
//         "--template-uri",
//         &url,
//     ];

//     if template.parameters.is_some() {
//         args.push("--parameters");

//         let p: Vec<&str> = parameters.iter().map(|s| &**s).collect();
//         args.extend(p);
//     }

//     run_az_command_with_output(args)
// }

fn run_az_command_with_output(args: Vec<&str>) -> Result<Output, AzCliError> {
    let az_cli_path = get_az_cli_path()?;
    debug!("Found the az CLI at {}", &az_cli_path.display());

    debug!("Running `az {}`", &args.join(" "));
    let output = cmd(az_cli_path, &args)
        .stderr_capture()
        .stdout_capture()
        .unchecked()
        .run()?;
    Ok(output)
}

fn find_command<T>(command: T) -> Option<PathBuf>
where
    T: AsRef<Path>,
{
    // If the command path is absolute and a file exists, then use that.
    if command.as_ref().is_absolute() && command.as_ref().is_file() {
        return Some(command.as_ref().to_path_buf());
    }
    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command.as_ref());
                if candidate.is_file() {
                    return Some(candidate);
                } else if let Some(result) = find_command_with_pathext(&candidate) {
                    return Some(result);
                }
            }
            None
        }
        None => None,
    }
}

fn find_command_with_pathext(candidate: &Path) -> Option<PathBuf> {
    if candidate.extension().is_none() {
        if let Some(pathexts) = env::var_os("PATHEXT") {
            for pathext in env::split_paths(&pathexts) {
                let mut source_candidate = candidate.to_path_buf();
                let extension = pathext.to_str().unwrap().trim_matches('.');
                source_candidate.set_extension(extension);
                let current_candidate = source_candidate.to_path_buf();
                if current_candidate.is_file() {
                    return Some(current_candidate);
                }
            }
        };
    }
    None
}
