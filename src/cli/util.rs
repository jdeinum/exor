use clap::{Args, Parser, Subcommand, arg};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Args)]
pub(crate) struct RunArgs {
    /// The path to your inventory directory
    #[arg(short = 'i')]
    pub(crate) inventory: String,

    /// The path to your playbook
    #[arg(short = 'p')]
    pub(crate) playbook: String,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Run(RunArgs),
}

pub(crate) fn parse_cli() -> Cli {
    Cli::parse()
}
