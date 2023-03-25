use dns_common::tracing::TracingConfig;

use crate::collection::Collection;
use crate::config;
use crate::platform::Platform;
use crate::prelude::*;
use dns_common::deser::yaml_from_path as from_path;

static YAML_FILE_NAME: &str = "config.yaml";

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parallelism {
    pub workers: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Igdb {
    pub api_base_url: String,
    pub images_base_url: String,
    pub denylisted_name_substrings: HashSet<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Twitch {
    pub client_id: String,
    pub client_secret: String,
    pub id_base_url: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct YamlConfig {
    pub parallelism: Parallelism,
    pub igdb: Igdb,
    pub twitch: Twitch,
    pub collections: Vec<Collection>,
    pub tracing: Option<TracingConfig>,
}

impl YamlConfig {
    pub fn new(env: &EnvConfig, clap: &ClapConfig) -> Result<Self> {
        if let Some(p) = clap.config.clone() {
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        if let Some(path_str) = env.config_path.as_ref() {
            let p = PathBuf::from(path_str);
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        let config_path = default_config_path();
        if let Ok(p) = config_path {
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        match clap.cmd {
            config::clap::Cmd::Config(_) => Ok(YamlConfig::default()),
            _ => Err(anyhow!(
                "No valid config. 
                
Please run the following to create a file with default values:
            
   {} config init",
                PROJECT_NAME
            )),
        }
    }
}

impl Default for YamlConfig {
    fn default() -> Self {
        Self {
            parallelism: Parallelism { workers: 4 },
            twitch: Twitch {
                client_id: "__CLIENT_ID__".into(),
                client_secret: "__CLIENT_SECRET__".into(),
                id_base_url: "https://id.twitch.tv".into(),
            },
            igdb: Igdb {
                api_base_url: "https://api.igdb.com".into(),
                images_base_url: "https://images.igdb.com".into(),
                denylisted_name_substrings: HashSet::new(),
            },
            collections: vec![
                Collection {
                    name: "Game Boy Advance".into(),
                    path: PathBuf::from("/path/to/ROMs/gba"),
                    extensions: vec!["zip".into()],
                    platform: Platform::GameBoyAdvance,
                    launch: "m start -n com.retroarch.aarch64/com.retroarch.browser.retroactivity.RetroActivityFuture -e ROM {file.path} -e LIBRETRO /data/data/com.retroarch.aarch64/cores/mgba_libretro_android.so -e CONFIGFILE /storage/emulated/0/Android/data/com.retroarch.aarch64/files/retroarch.cfg -e QUITFOCUS --activity-clear-task --activity-clear-top --activity-no-history".into(),
                    denylist: None,
                    title_map: None,
                },
            ],
            tracing: Some(TracingConfig {
                time: false,
                level: format!("{}=info", PROJECT_NAME),
            }),
        }
    }
}

pub fn default_config_path() -> Result<PathBuf> {
    super::default_dir().map(|p| p.join(YAML_FILE_NAME))
}
