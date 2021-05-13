use clipboard::*;
use std::collections::HashMap;
use translate::*;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(());
    }

    let string = args.iter()
        .flat_map(|s| s.trim().split_whitespace())
        .collect::<Vec<&str>>().join(" ");

    let lookup: Smol = Default::default();
    let translation = lookup.translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

struct Smol {
    lookup: HashMap<char, char>,
}

impl Smol {
    const CAPS: [char; 26] = [
        '\u{1D00}',  // LATIN_LETTER_SMALL_CAPITAL_A
        '\u{0299}',  // LATIN_LETTER_SMALL_CAPITAL_B
        '\u{1D04}',  // LATIN_LETTER_SMALL_CAPITAL_C
        '\u{1D05}',  // LATIN_LETTER_SMALL_CAPITAL_D
        '\u{1D07}',  // LATIN_LETTER_SMALL_CAPITAL_E
        '\u{A730}',  // LATIN_LETTER_SMALL_CAPITAL_F
        '\u{0262}',  // LATIN_LETTER_SMALL_CAPITAL_G
        '\u{029C}',  // LATIN_LETTER_SMALL_CAPITAL_H
        '\u{026A}',  // LATIN_LETTER_SMALL_CAPITAL_I
        '\u{1D0A}',  // LATIN_LETTER_SMALL_CAPITAL_J
        '\u{1D0B}',  // LATIN_LETTER_SMALL_CAPITAL_K
        '\u{029F}',  // LATIN_LETTER_SMALL_CAPITAL_L
        '\u{1D0D}',  // LATIN_LETTER_SMALL_CAPITAL_M
        '\u{0274}',  // LATIN_LETTER_SMALL_CAPITAL_N
        '\u{1D0F}',  // LATIN_LETTER_SMALL_CAPITAL_O
        '\u{1D18}',  // LATIN_LETTER_SMALL_CAPITAL_P
        '\u{A7AF}',  // LATIN_LETTER_SMALL_CAPITAL_Q
        '\u{0280}',  // LATIN_LETTER_SMALL_CAPITAL_R
        '\u{A731}',  // LATIN_LETTER_SMALL_CAPITAL_S
        '\u{1D1B}',  // LATIN_LETTER_SMALL_CAPITAL_T
        '\u{1D1C}',  // LATIN_LETTER_SMALL_CAPITAL_U
        '\u{1D20}',  // LATIN_LETTER_SMALL_CAPITAL_V
        '\u{1D21}',  // LATIN_LETTER_SMALL_CAPITAL_W
        // https://entropymine.wordpress.com/2018/05/26/the-curious-case-of-small-caps-in-unicode/
        '\u{1D605}',  // MATHEMATICAL SANS-SERIF BOLD SMALL X
        '\u{028F}',  // LATIN_LETTER_SMALL_CAPITAL_Y
        '\u{1D22}',  // LATIN_LETTER_SMALL_CAPITAL_Z
    ];
}

impl Default for Smol {
    fn default() -> Self {
        let lookup: HashMap<char, char> =
            ('a'..='z').chain('A'..='Z')
                .zip(Self::CAPS.iter().copied().chain(Self::CAPS.iter().copied()))
                .collect();

        Self { lookup }
    }
}

impl Lookup for Smol {
    fn lookup(&self) -> &HashMap<char, char> {
        &self.lookup
    }
}
