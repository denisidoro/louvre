use crate::prelude::*;
use serde::de;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Platform {
    Linux,
    Nintendo64,
    Wii,
    Windows,
    PlayStation,
    PlayStation2,
    PlayStation3,
    Xbox,
    Xbox360,
    Dos,
    Mac,
    Nes,
    SuperNintendo,
    NintendoDS,
    GameCube,
    GameBoyColor,
    Dreamcast,
    GameBoyAdvance,
    MegaDrive,
    GameBoy,
    Android,
    Nintendo3DS,
    Psp,
    WiiU,
    PlayStation4,
    WiiWare,
    Switch,
    NintendoDsi,
    Playstation5,
    XboxSeries,
    Arcade,
    Other(u8),
}

impl TryFrom<&str> for Platform {
    type Error = anyhow::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        use Platform::*;

        let id = v
            .to_lowercase()
            .replace(' ', "")
            .replace(|c: char| !c.is_alphanumeric(), "")
            .replace("sega", "")
            .replace("nintendo", "")
            .replace("microsoft", "")
            .replace("apple", "")
            .replace("sony", "");

        match id.as_str() {
            "lnx" | "linux" => Ok(Linux),
            "n64" | "64" => Ok(Nintendo64),
            "wii" => Ok(Wii),
            "arcade" | "cps" | "cps2" | "cps3" | "fbneo" | "fbalpha" | "fba" | "mame" => Ok(Arcade),
            "win" | "windows" | "pc" => Ok(Windows),
            "ps1" | "psx" | "playstation" => Ok(PlayStation),
            "ps2" | "playstation2" => Ok(PlayStation2),
            "ps3" | "playstation3" => Ok(PlayStation3),
            "xbox" => Ok(Xbox),
            "x360" | "xbox360" => Ok(Xbox360),
            "dos" => Ok(Dos),
            "mac" | "osx" | "macintosh" => Ok(Mac),
            "nes" => Ok(Nes),
            "snes" | "super" => Ok(SuperNintendo),
            "nds" | "ds" => Ok(NintendoDS),
            "ngc" | "gc" | "gamecube" => Ok(GameCube),
            "gbc" | "gameboycolor" => Ok(GameBoyColor),
            "dreamcast" => Ok(Dreamcast),
            "gba" | "gameboyadvance" => Ok(GameBoyAdvance),
            "smd" | "md" | "megadrive" => Ok(MegaDrive),
            "gb" | "gameboy" => Ok(GameBoy),
            "droid" | "android" => Ok(Android),
            "3ds" => Ok(Nintendo3DS),
            "psp" | "portable" => Ok(Psp),
            "wiiu" => Ok(WiiU),
            "ps4" | "playstation4" => Ok(PlayStation4),
            "wiiware" => Ok(WiiWare),
            "sw" | "ns" | "nsw" | "switch" => Ok(Switch),
            "dsi" => Ok(NintendoDsi),
            "ps5" | "playstation5" => Ok(Playstation5),
            "xsx" | "xss" | "xs" => Ok(XboxSeries),
            _ => match v.parse::<u8>() {
                Ok(n) => Ok(Other(n)),
                Err(e) => Err(e.into()),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(d)?;
        Platform::try_from(s.as_str())
            .map_err(|_| de::Error::custom(format!("Failed to deserialize platform: {}", s)))
    }
}
