use crate::platform::Platform;
use crate::prelude::*;
use derive_more::Display;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::convert::{From, TryFrom};

#[derive(Display)]
pub struct PlatformId(u8);

static MAPPING: Lazy<HashMap<u8, Platform>> = Lazy::new(|| {
    use Platform::*;
    HashMap::from([
        (3, Linux),
        (4, Nintendo64),
        (5, Wii),
        (6, Windows),
        (7, PlayStation),
        (8, PlayStation2),
        (9, PlayStation3),
        (11, Xbox),
        (12, Xbox360),
        (13, Dos),
        (14, Mac),
        (18, Nes),
        (19, SuperNintendo),
        (20, NintendoDS),
        (21, GameCube),
        (22, GameBoyColor),
        (23, Dreamcast),
        (24, GameBoyAdvance),
        (29, MegaDrive),
        (33, GameBoy),
        (34, Android),
        (37, Nintendo3DS),
        (38, Psp),
        (41, WiiU),
        (48, PlayStation4),
        (56, WiiWare),
        (130, Switch),
        (159, NintendoDsi),
        (167, Playstation5),
        (169, XboxSeries),
    ])
});

impl From<PlatformId> for Platform {
    fn from(i: PlatformId) -> Self {
        match MAPPING.get(&i.0) {
            Some(p) => *p,
            None => Platform::Other(i.0),
        }
    }
}

impl TryFrom<Platform> for PlatformId {
    type Error = anyhow::Error;

    fn try_from(platform: Platform) -> Result<Self, Self::Error> {
        match platform {
            Platform::Other(i) => Ok(PlatformId(i)),
            _ => {
                let kv = MAPPING.iter().find(|(_, v)| v == &&platform);
                let i = *kv.context("kv not found")?.0;
                Ok(PlatformId(i))
            }
        }
    }
}
