use std::collections::HashMap;

pub trait Lookup {
    fn lookup(&self) -> &HashMap<char, char>;
}

pub trait Translate: Lookup {
    fn translate(&self, string: &str) -> String;
}

impl<T> Translate for T
where
    T: Lookup,
{
    fn translate(&self, string: &str) -> String {
        let lookup = self.lookup();

        string
            .chars()
            .map(|c| lookup.get(&c).copied().unwrap_or(c))
            .collect()
    }
}
