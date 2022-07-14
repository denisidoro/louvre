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
        let cases = vec![
            (
                "Decrypted 1234 -   Legend of Zelda - Pokemon Bros. v1.2, The   (ROM) (ISO)",
                "The Legend of Zelda - Pokémon Bros",
            ),
            (
                "Monster Hunter 4 Ultimate Update v1.1 (EUR) ENC",
                "Monster Hunter 4 Ultimate",
            ),
            (
                "Phoenix Wright - Ace Attorney - Dual Destinies",
                "Phoenix Wright - Ace Attorney - Dual Destinies",
            ),
        ];

        for (input, output) in cases {
            assert_eq!(output, prettify(input));
        }
    }
}
