use crate::prelude::*;
use std::env;

pub struct EnvConfig {
    pub config_path: Option<String>,
}

impl Default for EnvConfig {
    fn default() -> Self {
        let config_env_var = format!("{}_CONFIG", PROJECT_NAME);
        Self {
            config_path: env::var(config_env_var).ok(),
        }
    }
}
