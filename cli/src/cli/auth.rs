use std::fmt::Display;

use clap_derive::{Args, ValueEnum};

use super::CliResult;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Platform {
    #[default]
    Anilist,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anilist => f.write_str("Anilist.co"),
        }
    }
}

#[derive(Args)]
pub struct Auth {
    #[clap(long, default_value_t = Platform::Anilist)]
    pub platform: Platform,
}

pub async fn execute(_auth: &Auth) -> CliResult<()> {
    Ok(())
}
