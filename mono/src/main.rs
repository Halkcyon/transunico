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

    let lookup: Mono = Default::default();
    let translation = lookup.translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

struct Mono {
    lookup: HashMap<char, char>,
}

impl Default for Mono {
    fn default() -> Self {
        let lookup = {
            let mut lookup: HashMap<char, char> =
                ('A'..='Z').chain('a'..='z').chain('0'..='9')
                    .zip(('𝙰'..='𝚣').chain('𝟶'..='𝟿'))
                    .collect();

            lookup.insert(' ', '\u{2002}');

            lookup
        };

        Self { lookup }
    }
}

impl Lookup for Mono {
    fn lookup(&self) -> &HashMap<char, char> {
        &self.lookup
    }
}
