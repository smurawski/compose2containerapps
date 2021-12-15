use custom_error::custom_error;
use duct::cmd;
use regex::Regex;
use serde_json::Value;
use std::env;
use std::io::{BufRead, BufReader};
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
}

fn get_az_cli_path() -> Result<PathBuf, AzCliError> {
    if let Some(cli_path) = find_command("az") {
        Ok(cli_path)
    } else {
        Err(AzCliError::CliMissing)
    }
}

#[derive(Clone, Debug)]
pub struct AzAccountInfo {
    subscription_name: Option<String>,
    subscription_id: Option<String>,
    tenant_id: Option<String>,
}

impl Default for AzAccountInfo {
    fn default() -> Self {
        AzAccountInfo {
            subscription_name: None,
            subscription_id: None,
            tenant_id: None,
        }
    }
}

pub fn set_azure_environment(subscription: &str) -> Result<(), AzCliError> {
    println!(
        "Checking to see if the Azure CLI is authenticated and which subscription is default."
    );
    let account = match get_account_info() {
        Ok(a) => a,
        Err(_) => {
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

// fn create_resource_group(command: &Command) -> Result<Output, AzCliError> {
//     let local_command = command.clone();
//     let rg = local_command.resource_group.unwrap();
//     let location = local_command.location.unwrap();
//     let args = vec!["group", "create", "--name", &rg, "--location", &location];
//     run_az_command_with_output(args)
// }

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

    let output = cmd(az_cli_path, &args)
        .stderr_to_stdout()
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
