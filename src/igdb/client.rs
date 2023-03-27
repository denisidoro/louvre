use super::*;
use crate::dns_common::deser::json_from_str;
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
    limit 4;"#,
            simple_name.trim(),
            platform_id.query_str(),
        );

        let uri = format!("{}/v4/games", self.config.yaml.igdb.api_base_url);
        let response = self.http.post(uri).body(body.clone()).send()?;
        let text = response.text()?;

        let games: Vec<Game> = json_from_str(&text)?;

        let denylisted_name_substrings: Vec<String> = self
            .config
            .yaml
            .igdb
            .denylisted_name_substrings
            .clone()
            .into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();

        let game_names = games
            .iter()
            .map(|game| game.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let res = best_result(games, &simple_name, denylisted_name_substrings)
            .with_context(|| format!("no valid game for body {}", body));

        match res {
            Ok(game) => {
                info!(
                    msg = "igdb game found",
                    query = simple_name,
                    game_names = game_names,
                    choice = &game.name,
                );
                Ok(game)
            }
            Err(e) => {
                error!(
                    msg = "igdb game not found",
                    query = simple_name,
                    game_names = game_names,
                );
                Err(e)
            }
        }
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

fn best_result(
    games: Vec<Game>,
    simple_name: &str,
    denylisted_name_substrings: Vec<String>,
) -> Option<Game> {
    if games.is_empty() {
        return None;
    }

    let mut scores = vec![127; games.len()];

    let version = get_number(simple_name).unwrap_or(1);

    for (i, game) in games.iter().enumerate() {
        scores[i] -= 12 * (i as u8);

        let game_name = game.name.to_ascii_lowercase();
        let this_version = get_number(&game_name).unwrap_or(1);

        for denylisted_name_substring in &denylisted_name_substrings {
            if game_name.contains(denylisted_name_substring) {
                info!(
                    status = "denylisted",
                    game = &game_name,
                    denylist_substr = &denylisted_name_substring
                );
                scores[i] /= 3;
            }
        }

        if version != this_version {
            scores[i] /= 2;
        }
    }

    let mut max_score: u8 = 0;
    let mut max_index = 0;
    for (i, &score) in scores.iter().enumerate() {
        if score > max_score {
            max_score = score;
            max_index = i;
        }
    }

    Some(games.get(max_index).expect("invalid game index").to_owned())
}

fn get_number(text: &str) -> Option<u8> {
    for split in text.split_whitespace() {
        if let Ok(n) = split.parse::<u8>() {
            return Some(n);
        }
    }
    None
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
        info!(bearer = &bearer);

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to init http client");

        Ok(Self { config, http })
    }
}

impl Component for Client {}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_games(names: Vec<&str>) -> Vec<Game> {
        names
            .into_iter()
            .enumerate()
            .map(|(i, name)| Game {
                id: (i as u32),
                name: name.to_owned(),
                ..Default::default()
            })
            .collect()
    }

    #[test]
    fn test_best_result() {
        let cases = vec![
            (
                vec![
                    "Pokemon Black Version 2",
                    "Pokemon Black",
                    "Pokemon White Version 2",
                ],
                "Pokemon Black Version",
                vec![""],
                1,
            ),
            (
                vec![
                    "Pokemon Black Version 2",
                    "Pokemon Black",
                    "Pokemon White Version 2",
                ],
                "Pokemon Black Version 2",
                vec![""],
                0,
            ),
            (
                vec!["dlroW oiraM repuS", "Super Mario World"],
                "Super Mario World",
                vec![""],
                0,
            ),
            (
                vec!["dlroW oiraM repuS", "Super Mario World"],
                "Super Mario World",
                vec!["dlroW"],
                1,
            ),
        ];

        for (game_names, simple_name, denylist, expected_index) in cases {
            let games = gen_games(game_names);
            let expected = games.get(expected_index).unwrap().name.to_owned();
            let denylist = denylist
                .into_iter()
                .map(|s| s.to_ascii_lowercase())
                .collect();
            let choice = best_result(games, simple_name, denylist);
            assert_eq!(expected, choice.unwrap().name)
        }
    }
}
