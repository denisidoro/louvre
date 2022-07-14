use super::*;
use crate::common::deser::json_from_str;
use crate::platform::Platform;
use crate::prelude::*;
use crate::twitch;
use reqwest::blocking::Client as HttpClient;
use reqwest::header;
use std::fs;
use std::io::Cursor;

pub struct Client {
    config: Arc<Config>,
    http: HttpClient,
}

impl Client {
    pub fn get_metadata(&self, name: &str, platform: Platform) -> Result<Game> {
        let platform_id: PlatformId = platform.try_into()?;

        let simple_name = name.replace('é', "e").replace(':', " ").replace(" - ", " ");

        let body = format!(
            r#"fields id,name,rating,aggregated_rating,cover.image_id,artworks.image_id,screenshots.image_id,first_release_date,genres.name,involved_companies.company.name,storyline,summary;
    search "{}";
    where platforms = ({});
    limit 1;"#,
            simple_name.trim(),
            platform_id,
        );

        let uri = format!("{}/v4/games", self.config.yaml.igdb.api_base_url);
        let response = self.http.post(uri).body(body.clone()).send()?;
        let text = response.text()?;

        let games: Vec<Game> = json_from_str(&text)?;

        games
            .into_iter()
            .next()
            .with_context(|| format!("empty result for body {}", body))
    }

    pub fn download(
        &self,
        image: &Image,
        size: ImageSize,
        is_retina: bool,
        path: &Path,
    ) -> Result<()> {
        let uri = format!(
            "{}/igdb/image/upload/t_{}{}/{}.jpg",
            self.config.yaml.igdb.images_base_url,
            size,
            if is_retina { "_2x" } else { "" },
            image.image_id
        );

        trace!(uri = &uri, path = path.to_string());

        let response = self.http.get(uri).send()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(&path)?;
        let mut content = Cursor::new(response.bytes()?);
        std::io::copy(&mut content, &mut file)?;

        Ok(())
    }
}

impl Client {
    pub fn new(config: Arc<Config>, twitch: &twitch::Client) -> Result<Self> {
        trace!("init");

        let client_id = config.yaml.twitch.client_id.clone();

        let access_token = &twitch.access_token;
        let bearer = format!("Bearer {}", access_token);

        let mut headers = header::HeaderMap::new();
        headers.insert("Client-ID", header::HeaderValue::from_str(&client_id)?);
        headers.insert("Authorization", header::HeaderValue::from_str(&bearer)?);

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to init http client");

        Ok(Self { config, http })
    }
}

impl Component for Client {}
