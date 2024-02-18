use cli::CliResult;

pub mod anilist;
pub mod cli;

#[tokio::main]
async fn main() -> CliResult<()> {
    cli::run().await
}
