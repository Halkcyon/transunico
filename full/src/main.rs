use clipboard::*;
use std::collections::HashMap;
use translate::*;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(());
    }

    let lookup: FullWidth = Default::default();

    let string = args.iter()
        .flat_map(|s| s.trim().split_whitespace())
        .collect::<Vec<&str>>().join(" ");
    let string = lookup.sub(&string);

    let translation = lookup.translate(&string);

    Clipboard::new().set_clipboard(&translation)?;

    Ok(())
}

struct FullWidth {
    lookup: HashMap<char, char>,
}

impl FullWidth {
    const PLAINTEXT: &'static str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

    const UNICODE: &'static str = "\u{2002}！＂＃＄％＆＇（）＊＋，－．／０１２３４５６７８９：；＜＝＞？＠ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ［＼］＾＿｀ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ｛｜｝～";

    fn sub(&self, s: &str) -> String {
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
    }
}

impl Default for FullWidth {
    fn default() -> Self {
        Self {
            lookup: Self::PLAINTEXT.chars().zip(Self::UNICODE.chars()).collect(),
        }
    }
}

impl Lookup for FullWidth {
    fn lookup(&self) -> &HashMap<char, char> {
        &self.lookup
    }
}
