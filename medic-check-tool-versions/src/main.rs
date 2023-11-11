use medic_check_tool_versions::cli::{CliArgs, Command as Cmd};
use medic_lib::CheckResult;

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Cmd::PackageInstalled(args) => {
            medic_check_tool_versions::package_installed(args.plugin, args.version)
        }
        Cmd::PluginInstalled(args) => medic_check_tool_versions::plugin_installed(args.plugin),
    }
}
