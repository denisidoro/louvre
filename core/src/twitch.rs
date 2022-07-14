use crate::config;
use crate::prelude::*;
use common::deser::json_from_str;
use common::deser::to_yaml_str;
use common::deser::yaml_from_path;
use reqwest::blocking::Client as HttpClient;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

static FILE_NAME: &str = "auth.yaml";

#[derive(Serialize, Deserialize)]
struct Response {
    access_token: String,
    expires_in: u32, // secs
    token_type: String,
}

#[derive(Serialize, Deserialize)]
struct Auth {
    timestamp: u64, // secs
    response: Response,
}

impl Auth {
    fn is_expired(&self) -> bool {
        let now = now_in_secs();
        let expiry = self.timestamp + (self.response.expires_in as u64);

        expiry <= now || expiry - now <= 60 * 5
    }
}

pub struct Client {
    pub access_token: String,
}

impl Client {
    pub fn new(config: &Config) -> Result<Self> {
        trace!("init");

        let auth_path = config::default_dir()?.join(FILE_NAME);

        let access_token = if !auth_path.exists() {
            refresh_token(config, &auth_path)?
        } else {
            let auth: Auth = yaml_from_path(&auth_path)?;
            if auth.is_expired() {
                refresh_token(config, &auth_path)?
            } else {
                auth.response.access_token
            }
        };

        Ok(Self { access_token })
    }
}

fn refresh_token(config: &Config, auth_path: &Path) -> Result<String> {
    let twitch_config = &config.yaml.twitch;
    let uri = format!(
        "{}/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        twitch_config.id_base_url, twitch_config.client_id, twitch_config.client_secret
    );
    trace!(uri);

    let client = HttpClient::new();
    let res = client.post(uri).send()?;
    let text = res.text()?;
    trace!(text);

    let response: Response = json_from_str(&text)?;

    let auth = Auth {
        timestamp: now_in_secs(),
        response,
    };

    let serialized = to_yaml_str(&auth)?;

    let parent = auth_path.parent().context("no parent")?;
    fs::create_dir_all(parent)?;
    fs::write(&auth_path, serialized)?;

    Ok(auth.response.access_token)
}

impl Component for Client {}

fn now_in_secs() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("negative time");
    since_the_epoch.as_secs()
}
