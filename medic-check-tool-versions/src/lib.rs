#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::CheckResult::{self, CheckError, CheckOk};
use medic_lib::std_to_string;

use std::process::Command;

pub mod cli;

enum RuntimeManager {
    Asdf,
    Mise,
}

static MISE_CORE_PLUGINS: [&str; 14] = [
    "bun", "deno", "elixir", "erlang", "go", "golang", "java", "node", "nodejs", "python", "ruby",
    "rust", "swift", "zig",
];

impl std::fmt::Display for RuntimeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeManager::Asdf => write!(f, "asdf"),
            RuntimeManager::Mise => write!(f, "mise"),
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
                format!("Runtime manager package ({rtm}) is missing installed version: {plugin}."),
                Some(stdout),
                Some(stderr),
                Some(format!("{remedy:?}").replace('"', "")),
            )
        }
    } else {
        fail_no_rtm()
    }
}

pub fn plugin_installed(plugins: Vec<String>) -> CheckResult {
    if let Ok(rtm) = installed_runtime_manager() {
        let mut missing: Vec<String> = vec![];

        let mut command = Command::new(format!("{rtm}"));
        command.args(["plugin", "list"]);
        let list = command.output().unwrap();
        let plugin_list = std_to_string(list.stdout);
        let installed: Vec<String> = plugin_list.split('\n').map(str::to_string).collect();

        for plugin in plugins {
            match rtm {
                RuntimeManager::Asdf => (),
                RuntimeManager::Mise => {
                    if MISE_CORE_PLUGINS.contains(&&*plugin) {
                        continue;
                    }
                }
            }

            if !installed.contains(&plugin) {
                missing.push(plugin);
            }
        }

        if missing.is_empty() {
            CheckOk
        } else {
            let remedies: Vec<String> = missing
                .iter()
                .map(|plugin| match rtm {
                    RuntimeManager::Asdf => format!("{rtm} plugin add {plugin}"),
                    RuntimeManager::Mise => format!("{rtm} plugin install {plugin}"),
                })
                .collect();

            CheckError(
                format!(
                    "Runtime manager ({rtm}) is missing plugins: {}.",
                    missing.join(", ")
                ),
                Some(plugin_list),
                None,
                Some(remedies.join(" && ")),
            )
        }
    } else {
        fail_no_rtm()
    }
}

fn installed_runtime_manager() -> Result<RuntimeManager, ()> {
    let which_mise = Command::new("which").args(["mise"]).output().unwrap();
    if which_mise.status.success() {
        return Ok(RuntimeManager::Mise);
    }

    let which_asdf = Command::new("which").args(["asdf"]).output().unwrap();
    if which_asdf.status.success() {
        return Ok(RuntimeManager::Asdf);
    }

    Err(())
}

fn fail_no_rtm() -> CheckResult {
    CheckError(
        "Unable to find a runtime manager.".into(),
        None,
        Some("Neither ASDF nor MISE were found in the current shell".into()),
        None,
    )
}
