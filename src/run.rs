use crate::error::Result;
use crate::{
    cli::util::{Commands, parse_cli},
    system,
};
use std::process::exit;
use tracing::error;

/// The main entrypoint in our library
pub async fn run() -> Result<()> {
    // enable tracing
    tracing_subscriber::fmt::init();

    // parse the cli
    let cli = parse_cli();

    // run the command
    let res = match &cli.command {
        Commands::Run(run_args) => system::run(run_args).await,
    };

    // log any errors
    if let Err(e) = res {
        error!("Error running: {e:?}");
        exit(1);
    }

    Ok(())
}
