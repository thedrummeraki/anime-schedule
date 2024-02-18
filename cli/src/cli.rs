use clap::{Parser, Subcommand};

use self::find::Find;

pub mod auth;
pub mod find;
pub mod prompt;

#[derive(Debug)]
pub enum CliError {}
pub type CliResult<T> = Result<T, CliError>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Find(Find),
}

pub async fn run() -> CliResult<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Find(find) => find::execute(find).await?,
    }
    Ok(())
}
