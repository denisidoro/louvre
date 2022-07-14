use std::env;

use crate::prelude::*;

static RUST_LOG: &str = "RUST_LOG";

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TracingConfig {
    pub time: bool,
    pub level: String,
}

pub fn init(config: Option<&TracingConfig>) {
    if let Some(tracing) = config {
        let level_is_set = match env::var(RUST_LOG) {
            Err(_) => false,
            Ok(v) => !v.is_empty(),
        };

        if !level_is_set {
            env::set_var(RUST_LOG, &tracing.level);
        }

        let t = tracing_subscriber::fmt();
        let res = if tracing.time {
            t.try_init()
        } else {
            t.without_time().try_init()
        };

        if let Err(e) = res {
            error!("unable to set tracing subscriber: {}", e);
        }
    }
}
