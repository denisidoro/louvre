use crate::platform::Platform;
use crate::prelude::*;
use serde::ser;

pub fn id(platform: &Platform) -> Result<&'static str> {
    use Platform::*;
    match platform {
        Android => Ok("android"),
        Dos => Ok("dos"),
        GameBoyAdvance => Ok("gba"),
        GameBoyColor => Ok("gbc"),
        GameBoy => Ok("gb"),
        Linux => Ok("linux"),
        Mac => Ok("macintosh"),
        Nintendo3DS => Ok("3ds"),
        Nintendo64 => Ok("n64"),
        NintendoDS => Ok("nds"),
        SuperNintendo => Ok("snes"),
        Nes => Ok("nes"),
        GameCube => Ok("gc"),
        Switch => Ok("switch"),
        Wii => Ok("wii"),
        WiiU => Ok("wiiu"),
        PlayStation => Ok("psx"),
        PlayStation2 => Ok("ps2"),
        PlayStation3 => Ok("ps3"),
        Psp => Ok("psp"),
        Dreamcast => Ok("dreamcast"),
        Windows => Ok("windows"),
        Xbox => Ok("xbox"),
        Xbox360 => Ok("xbox360"),
        _ => Err(anyhow!("foo")),
    }
}

impl Serialize for Platform {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id = id(self).map_err(|_e| ser::Error::custom("Failed to serialize platform"))?;
        s.serialize_str(id)
    }
}

// 3DO "3do"
// Amiga "amiga"
// Amiga CD32 "amigacd32"
// Amiga CDTV "amigacdtv"
// Amstrad CPC "amstradcpc"
// Apple II "apple2"
// Arcade "arcade"
// Arcade (Daphne) "daphne"
// Arcade (Final Burn Alpha) "fba"
// Arcade (MAME) "mame"
// Atari 800 "atari800"
// Atari 2600 "atari2600"
// Atari 5200 "atari5200"
// Atari 7800 "atari7800"
// Atari Jaguar "atarijaguar"
// Atari Jaguar CD "atarijaguarcd"
// Atari Lynx "atarilynx"
// Atari ST "atarist"
// Atari XE "atarixe"
// CHIP-8 "chip8"
// ColecoVision "colecovision"
// Commodore 64 "c64"
// Dragon 32 "dragon32"
// Famicom Disk System "fds"
// GOG "gog"
// Intellivision "intellivision"
// Löve "love"
// MSX "msx"
// Naomi "naomi"
// Neo Geo CD "neogeocd"
// Neo Geo Pocket Color "ngpc"
// Neo Geo Pocket "ngp"
// Neo Geo "neogeo"
// Nintendo Game-and-Watch "gameandwatch"
// Nintendo VirtualBoy "virtualboy"
// Odyssey 2 "odyssey2"
// PC 88 "pc88"
// PC 98 "pc98"
// PC Engine "pcengine"
// PC "pc"
// PC-FX "pcfx"
// PlayStation Vita "psvita"
// RPG Maker "rpgmaker"
// SAM coupe "samcoupe"
// Scumm VM "scummvm"
// SEGA 32X "sega32x"
// SEGA CD "segacd"
// SEGA GameGear "gamegear"
// Sega Genesis "genesis"
// Sega Master System "mastersystem"
// Sega Megadrive "megadrive"
// Sega Saturn "saturn"
// SEGA SG-1000 "sg1000"
// Sharp X1 "x1"
// Sharp X6800 "x68000"
// Steam "steam"
// Super Nintendo Entertainment System "snes"
// SuperGrafx "supergrafx"
// TIC80 "tic80"
// TurboGrafx 16 "turbografx16"
// Vectrex "vectrex"
// WonderSwan "wonderswan"
// WonderSwan/Color "wonderswancolor"
// ZX Spectrum "zxspectrum"
// ZX81 "zx81"
