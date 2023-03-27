use crate::collection::Collection;
use crate::meta::{self, Meta};
use crate::prelude::*;
use chrono::prelude::*;
use std::cmp::max;
use std::fmt::Write as _;

pub fn to_str(meta: Meta, meta_file: &Path, _collection: &Collection) -> Result<String> {
    let mut buf = String::new();
    let rom = meta_file
        .file_name()
        .expect("no filename")
        .to_string_lossy()
        .replace(meta::YAML_NAME, "");

    writeln!(buf, "game: {}", &meta.igdb.name)?;

    writeln!(buf, "file: ../{}", &meta.file.to_string())?;

    if let Some(secs) = meta.igdb.first_release_date {
        let naive = NaiveDateTime::from_timestamp_opt(secs as i64, 0).context("invalid date")?;
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        writeln!(buf, "release: {}", datetime.format("%Y-%m-%d"))?
    };

    if let Some(x) = meta.igdb.genres {
        let genres: Vec<_> = x.iter().map(|g| g.name.clone()).collect();
        if !genres.is_empty() {
            writeln!(buf, "genres: {}", genres.join(", "))?
        }
    };

    writeln!(buf, "assets.boxFront: {}boxFront.jpg", rom)?;
    writeln!(buf, "assets.background: {}background.jpg", rom)?;
    writeln!(buf, "assets.screenshot: {}screenshot.jpg", rom)?;

    if let Some(x) = meta.igdb.aggregated_rating.or(meta.igdb.rating) {
        writeln!(buf, "rating: {}%", x as u8)?;
    };

    if let Some(companies) = &meta.igdb.involved_companies {
        let devs: Vec<String> = companies
            .iter()
            .filter(|x| x.developer.unwrap_or(false))
            .map(|x| x.company.name.clone())
            .collect();

        if !devs.is_empty() {
            writeln!(buf, "developers: {}", devs.join(", "))?
        }
    };

    if let Some(companies) = meta.igdb.involved_companies {
        let publishers: Vec<String> = companies
            .into_iter()
            .filter(|x| x.publisher.unwrap_or(false))
            .map(|x| x.company.name)
            .collect();

        if !publishers.is_empty() {
            writeln!(buf, "publishers: {}", publishers.join(", "))?
        }
    };

    writeln!(
        buf,
        "players: {}",
        meta.igdb
            .multiplayer_modes
            .unwrap_or_default()
            .iter()
            .map(|mode| {
                max(
                    mode.offlinemax.unwrap_or(1),
                    mode.offlinecoopmax.unwrap_or(1),
                )
            })
            .max()
            .unwrap_or(1)
    )?;

    if let Some(x) = meta.igdb.summary {
        writeln!(buf, "summary: {}", x.replace('\n', "\\n"))?
    };

    if let Some(x) = meta.igdb.storyline {
        writeln!(buf, "description: {}", x.replace('\n', "\\n"))?
    };

    Ok(buf)
}
