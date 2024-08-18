use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-tool-versions")]
/// Checks for whether runtime manager dependencies are available.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks whether a package for a given plugin is installed.
    PackageInstalled(RTMPackageArgs),
    /// Checks whether a plugin is installed.
    PluginInstalled(RTMPluginArgs),
}

#[derive(Args, Debug)]
pub struct RTMPackageArgs {
    /// Name of an ASDF-compatible plugin.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub plugin: String,

    /// Version of an ASDF-compatible package to install.
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub version: Option<String>,
}

#[derive(Args, Debug)]
pub struct RTMPluginArgs {
    /// Name of an ASDF-compatible plugin.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub plugin: Vec<String>,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
