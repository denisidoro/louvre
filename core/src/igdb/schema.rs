use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: u32,
    pub image_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiplayerMode {
    pub offlinemax: Option<u32>,
    pub offlinecoopmax: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyDoc {
    pub company: Company,
    pub developer: Option<bool>,
    pub publisher: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub aggregated_rating: Option<f32>,
    pub rating: Option<f32>,
    pub artworks: Option<Vec<Image>>,
    pub screenshots: Option<Vec<Image>>,
    pub cover: Option<Image>,
    pub first_release_date: Option<u32>,
    pub genres: Option<Vec<Genre>>,
    pub involved_companies: Option<Vec<CompanyDoc>>,
    pub storyline: Option<String>,
    pub summary: Option<String>,
    pub multiplayer_modes: Option<Vec<MultiplayerMode>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ImageSize {
    CoverSmall,
    ScreenshotMed,
    CoverBig,
    LogoMed,
    ScreenshotBig,
    ScreenshotHuge,
    Thumb,
    Micro,
    Res720p,
    Res1080p,
}

impl fmt::Display for ImageSize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use ImageSize::*;
        let text = match self {
            CoverSmall => "cover_small",
            ScreenshotMed => "screenshot_med",
            CoverBig => "cover_big",
            LogoMed => "logo_med",
            ScreenshotBig => "screenshot_big",
            ScreenshotHuge => "screenshot_huge",
            Thumb => "thumb",
            Micro => "micro",
            Res720p => "720p",
            Res1080p => "1080p",
        };
        fmt.write_str(text)?;
        Ok(())
    }
}
