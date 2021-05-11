use clipboard::{Clipboard, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(());
    }

    let lookup = FullWidth::new();

    let string = args.iter()
        .flat_map(|s| s.trim().split_whitespace())
        .collect::<Vec<&str>>().join(" ");

    let translation = lookup.translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

struct FullWidth {
    map: HashMap<char, char>,
}

impl FullWidth {
    const PLAINTEXT: &'static str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

    const UNICODE: &'static str = "\u{2002}！＂＃＄％＆＇（）＊＋，－．／０１２３４５６７８９：；＜＝＞？＠ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ［＼］＾＿｀ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ｛｜｝～";

    fn new() -> Self {
        Default::default()
    }

    fn translate(&self, s: &str) -> String {
        s
            .replace("! ", "!")
            .replace(" (", "(")
            .replace(") ", ")")
            .replace(", ", ",")
            .replace(". ", ".")
            .replace(": ", ":")
            .replace("; ", ";")
            .replace("? ", "?")
            .replace(" [", "[")
            .replace("] ", "]")
            .replace(" {", "{")
            .replace("} ", "}")
            .chars()
            .map(|c| self.map.get(&c).copied().unwrap_or(c))
            .collect()
    }
}

impl Default for FullWidth {
    fn default() -> Self {
        Self {
            map: Self::PLAINTEXT.chars().zip(Self::UNICODE.chars()).collect(),
        }
    }
}
