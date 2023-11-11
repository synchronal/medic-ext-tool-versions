#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};

use std::process::Command;

pub mod cli;

enum RuntimeManager {
    Asdf,
    Rtx,
}

static RTX_CORE_PLUGINS: [&str; 7] = ["go", "golang", "java", "node", "nodejs", "python", "ruby"];

impl std::fmt::Display for RuntimeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeManager::Asdf => write!(f, "asdf"),
            RuntimeManager::Rtx => write!(f, "rtx"),
        }
    }
}

pub fn package_installed(plugin: String, version: Option<String>) -> CheckResult {
    if let Ok(rtm) = installed_runtime_manager() {
        let mut command = Command::new(format!("{rtm}"));
        let mut remedy = Command::new(format!("{rtm}"));

        command.arg("where").arg(&plugin);
        remedy.arg("install").arg(&plugin);

        if let Some(package_version) = version {
            command.arg(&package_version);
            remedy.arg(&package_version);
        }

        let output = command.output().unwrap();
        if output.status.success() {
            CheckOk
        } else {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            CheckError(
                    format!("Currently configured runtime manager package for {plugin} has not been installed."),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("({remedy:?})").replace('"', "")),
                )
        }
    } else {
        fail_no_rtm()
    }
}

pub fn plugin_installed(plugin: String) -> CheckResult {
    if let Ok(rtm) = installed_runtime_manager() {
        match rtm {
            RuntimeManager::Asdf => (),
            RuntimeManager::Rtx => {
                if RTX_CORE_PLUGINS.contains(&&*plugin) {
                    return CheckOk;
                }
            }
        }
        let mut command = Command::new(format!("{rtm}"));
        command.args(["plugin", "list"]);

        let list = command.output().unwrap();
        let plugin_list = std_to_string(list.stdout);
        let plugins: Vec<String> = plugin_list.split('\n').map(str::to_string).collect();
        if plugins.contains(&plugin) {
            CheckOk
        } else {
            CheckError(
                format!("Runtime manager plugin {plugin} has not been installed."),
                Some(plugin_list),
                None,
                Some(format!("{rtm} plugin install {plugin}")),
            )
        }
    } else {
        fail_no_rtm()
    }
}

fn installed_runtime_manager() -> Result<RuntimeManager, ()> {
    let which_asdf = Command::new("which").args(["asdf"]).output().unwrap();
    if which_asdf.status.success() {
        return Ok(RuntimeManager::Asdf);
    }
    let which_rtx = Command::new("which").args(["rtx"]).output().unwrap();
    if which_rtx.status.success() {
        return Ok(RuntimeManager::Rtx);
    }

    Err(())
}

fn fail_no_rtm() -> CheckResult {
    CheckError(
        "Unable to find a runtime manager.".into(),
        None,
        Some("Neither ASDF nor RTX were found in the current shell".into()),
        None,
    )
}
