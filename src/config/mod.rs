mod clap;
mod env;
pub mod yaml;

pub use self::clap::ClapConfig;
pub use self::env::EnvConfig;
pub use self::yaml::YamlConfig;

use crate::prelude::*;

pub struct Config {
    pub cli: ClapConfig,
    pub yaml: YamlConfig,
}

impl Config {
    pub fn new(args: Option<Vec<&str>>) -> Result<Self> {
        let cli = ClapConfig::new(args)?;
        let env = EnvConfig::default();
        let yaml = YamlConfig::new(&env, &cli)?;
        Ok(Self { cli, yaml })
    }
}

pub fn default_dir() -> Result<PathBuf> {
    dns_common::fs::config_dir(PROJECT_NAME)
}
