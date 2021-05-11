use clipboard::{Clipboard, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(());
    }

    let lookup = if args[0] == "--bold" || args[0] == "-b" {
        args.remove(0);
        Blackletter::new(true)
    } else {
        Blackletter::new(false)
    };

    let string = args.iter()
        .flat_map(|s| s.trim().split_whitespace())
        .collect::<Vec<&str>>().join(" ");

    let translation = lookup.translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

struct Blackletter {
    map: HashMap<char, char>,
}

impl Blackletter {
    fn new(bold: bool) -> Self {
        let map =
            ('A'..='Z').chain('a'..='z')
                .zip(if bold {
                    '\u{1D56C}'..='\u{1D59F}'
                } else {
                    '\u{1D504}'..='\u{1D537}'
                })
                .collect();

        Self { map }
    }

    fn translate(&self, s: &str) -> String {
        s.chars().map(|c| if self.map.contains_key(&c) {
            self.map[&c]
        } else {
            c
        })
        .collect()
    }
}
