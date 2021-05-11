use clipboard::{Clipboard, Error};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static LOOKUP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut lookup: HashMap<char, char> =
        ('A'..='Z').chain('a'..='z').chain('0'..='9')
            .zip(('𝙰'..='𝚣').chain('𝟶'..='𝟿'))
            .collect();

    lookup.insert(',', '❟');
    lookup.insert('!', '❢');
    lookup.insert(' ', '\u{2002}');

    lookup
});

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(());
    }

    let string = args.iter()
        .flat_map(|s| s.trim().split_whitespace())
        .collect::<Vec<&str>>().join(" ");

    let translation = translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

fn translate(s: &str) -> String {
    s.chars().map(|c| if LOOKUP.contains_key(&c) {
        LOOKUP[&c]
    } else {
        c
    })
    .collect()
}
