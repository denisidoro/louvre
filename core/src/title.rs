use crate::prelude::*;

static REGEXES: Lazy<[Regex; 7]> = Lazy::new(|| {
    [
        regex(r"\([^\)]+\)"),     // remove (...)
        regex(r"/\[[^\]]+\]"),    // remove [...]
        regex(r"^[0-9]+ \-"),     // remove 1234 -
        regex(r"v[0-9\.]+"),      // remove v1.2
        regex(r" +"),             // remove consecutive spaces
        regex(r"(.*), ?The *$"),  // remove leading ", The"
        regex(r"\w{4}\d{5}\-\["), // remove BLES00539-[
    ]
});

pub fn prettify(path: &Path) -> String {
    let grandparent = path.parent().and_then(|p| p.parent());
    let actual_path = match grandparent {
        Some(gp) => {
            let gp_name = gp
                .file_stem()
                .unwrap_or_default()
                .to_string()
                .to_ascii_uppercase();
            if gp_name == "PS3_GAME" {
                gp.parent().unwrap_or(path)
            } else {
                path
            }
        }
        None => path,
    };

    let t = actual_path
        .file_stem()
        .map(|f| f.to_string())
        .unwrap_or_default()
        .trim_start_matches("Decrypted")
        .trim_end_matches(" (ROM)")
        .trim_end_matches(" (ISO)")
        .trim_end_matches(']')
        .replace("Bros.", "Bros")
        .replace("Pokemon", "Pokémon")
        .replace("Legend of Zelda, The", "The Legend of Zelda");

    let t = REGEXES[6].replace_all(t.trim(), "");
    let t = REGEXES[0].replace_all(t.trim(), "");
    let t = REGEXES[1].replace_all(t.trim(), "");
    let t = REGEXES[2].replace_all(t.trim(), "");
    let t = REGEXES[3].replace_all(t.trim(), "");
    let t = REGEXES[4].replace_all(t.trim(), " ");
    let t = REGEXES[5].replace_all(t.trim(), "The $1");

    t.trim()
        .trim_end_matches(" ENC")
        .trim()
        .trim_end_matches(" Update")
        .trim()
        .into()
}

fn regex(re: &str) -> Regex {
    Regex::new(re).expect("invalid regex")
}

#[cfg(test)]
mod tests {
    use super::*;
    use walkdir::WalkDir;

    #[test]
    fn test_prettify() {
        let dir = PathBuf::from("../fixtures/ROMs");
        let entries = WalkDir::new(&dir)
            .max_depth(6)
            .into_iter()
            .filter_map(|e| e.ok());

        let allowlist = ["zip", "rar", "7z", "iso", "cue", "bin", "3ds", "cia"];

        let mut output = vec![];
        for entry in entries {
            let path = entry.path();
            let extension = path
                .extension()
                .unwrap_or_default()
                .to_string()
                .to_ascii_lowercase();
            if allowlist.contains(&extension.as_str()) {
                output.push(prettify(path));
            }
        }

        output.sort();
        output.dedup();

        assert_eq!(
            output,
            [
                "Animal Crossing - New Leaf",
                "Dragon Ball Z - Budokai Tenkaichi 3",
                "Dragon Ball Z - Tenkaichi Tag Team",
                "Elite Beat Agents",
                "Fullmetal Alchemist Stray Rondo",
                "Ghost Trick - Phantom Detective",
                "God of War - Chains of Olympus",
                "Kingdom Hearts 3D - Dream Drop Distance",
                "Luigi's Mansion",
                "Mega Man X4",
                "Metroid Prime",
                "Monster Hunter 4",
                "Monster Hunter 4 Ultimate",
                "Monster Hunter Freedom Unite",
                "Monster Hunter Tri",
                "New Super Mario Bros",
                "Paper Mario - The Thousand-Year Door",
                "Persona 5",
                "Phoenix Wright - Ace Attorney - Dual Destinies",
                "Pikmin",
                "Pokémon - Version Cristal",
                "Pokémon - Version Jaune - Edition Speciale Pikachu",
                "Pokémon Emerald",
                "Pokémon FireRed",
                "Pokémon Mystery Dungeon - Gates to Infinity",
                "Pokémon Trading Card Game",
                "Pokémon X",
                "Professor Layton and the Miracle Mask",
                "Super Mario Galaxy",
                "Super Mario Sunshine",
                "Super Smash Bros Melee",
                "The Legend of Zelda - A Link to the Past",
                "The Legend of Zelda - Majora's Mask",
                "The Legend of Zelda - Oracle of Seasons",
                "The Legend of Zelda - Phantom Hourglass",
                "The Legend of Zelda - Spirit Tracks",
                "The Legend of Zelda - The Minish Cap",
                "The Legend of Zelda - The Wind Waker",
                "The World Ends With You",
                "Uncharted 3 Drakes Deception",
                "scph1001"
            ]
        );
    }
}
