use crate::config;
use crate::prelude::*;
use clap::Args;
use clap::Subcommand;
use common::deser::to_yaml_str;
use std::fs;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Init,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

impl Input {
    fn handle_init(&self) -> Result<()> {
        let yaml = YamlConfig::default();
        let yaml_str = to_yaml_str(&yaml)?;
        let path = config::yaml::default_config_path()?;

        let parent = path.parent().context("no parent")?;
        fs::create_dir_all(parent)?;
        fs::write(&path, yaml_str)?;

        eprintln!(
            "File created with default values.
            
Please run the following to edit the recently created config file:
        
   $EDITOR \"{}\"",
            path.to_string()
        );

        Ok(())
    }
}

impl Runnable for Input {
    fn run(&self, _system: System) -> Result<()> {
        match self.subcmd {
            SubCmd::Init => self.handle_init(),
        }
    }
}

impl HasDeps for Input {}
