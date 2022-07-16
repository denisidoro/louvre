use crate::prelude::*;


static REGEXES: Lazy<[Regex; 6]> = Lazy::new(|| {
    [
        regex(r"\([^\)]+\)"),    // remove (...)
        regex(r"/\[[^\]]+\]"),   // remove [...]
        regex(r"^[0-9]+ \-"),    // remove 1234 -
        regex(r"v[0-9\.]+"),     // remove v1.2
        regex(r" +"),            // remove consecutive spaces
        regex(r"(.*), ?The *$"), // remove leading ", The"
    ]
});

pub fn prettify(title: &str) -> String {
    let t = title
        .trim_start_matches("Decrypted")
        .trim_end_matches(" (ROM)")
        .trim_end_matches(" (ISO)")
        .replace("Bros.", "Bros")
        .replace("Pokemon", "Pokémon")
        .replace("Legend of Zelda, The", "The Legend of Zelda");

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

    #[test]
    fn test_prettify() {
        let dir = PathBuf::from("../fixtures/ROMs");
        let entries = WalkDir::new(&dir)
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok());

        let mut output = vec![];
        for entry in entries {
            let path = entry.path();
            let extension = path.extension().unwrap_or_default().to_string();
            if ["zip", "rar", "7z", "iso"].contains(&extension.as_str()) {
                let original_title = path.file_stem().unwrap_or_default().to_string();
                output.push(prettify(&original_title));
            }
        }

        output.sort();
        output.dedup();

        assert_eq!(
            output,
            [
                "Dragon Ball Z - Budokai Tenkaichi 3",
                "Dragon Ball Z - Tenkaichi Tag Team",
                "Elite Beat Agents",
                "Fullmetal Alchemist Stray Rondo",
                "Ghost Trick - Phantom Detective",
                "God of War - Chains of Olympus",
                "Luigi's Mansion",
                "Metroid Prime",
                "Monster Hunter Freedom Unite",
                "Monster Hunter Tri",
                "New Super Mario Bros",
                "Paper Mario - The Thousand-Year Door",
                "Pikmin",
                "Pokémon - Version Cristal",
                "Pokémon - Version Jaune - Edition Speciale Pikachu",
                "Pokémon Emerald",
                "Pokémon FireRed",
                "Pokémon Trading Card Game",
                "Super Mario Galaxy",
                "Super Mario Sunshine",
                "Super Smash Bros Melee",
                "The Legend of Zelda - A Link to the Past",
                "The Legend of Zelda - Oracle of Seasons",
                "The Legend of Zelda - Phantom Hourglass",
                "The Legend of Zelda - Spirit Tracks",
                "The Legend of Zelda - The Minish Cap",
                "The Legend of Zelda - The Wind Waker",
                "The World Ends With You"
            ]
        );
    }
}
