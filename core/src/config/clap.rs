use crate::commands;
use crate::prelude::*;
use clap::{Parser, Subcommand};
use common_derive::{HasDeps, Runnable};

#[derive(Subcommand, Debug, Clone, Runnable, HasDeps)]
pub enum Cmd {
    Scrape(commands::scrape::Input),
    Media(commands::media::Input),
    Pegasus(commands::pegasus::Input),
    Config(commands::config::Input),
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct ClapConfig {
    #[clap(subcommand)]
    pub cmd: Cmd,

    #[clap(short, long)]
    pub config: Option<PathBuf>,
}

impl ClapConfig {
    pub fn new(args: Option<Vec<&str>>) -> Result<Self> {
        // dbg!(&args);
        match args {
            Some(a) => Self::try_parse_from(&a).map_err(|e| e.into()),
            None => Self::try_parse().map_err(|e| e.into()),
        }
    }
}
