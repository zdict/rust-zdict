use super::{Lookup, Display};

pub struct Dict;
impl Lookup for Dict {
    const HOMEPAGE_URL: &'static str = "https://tw.dictionary.yahoo.com/";
    const API: &'static str = "https://tw.dictionary.yahoo.com/dictionary?p={word}";
    const TITLE: &'static str = "Yahoo Dictionary";
    const PROVIDER: &'static str = "yahoo";
    type Record = Record;
    fn query(&self, _url: &str) -> Self::Record {
        Record { content: "content" }
    }
}

pub struct Record {
    content: &'static str,
}
impl Display for Record {
    fn show(&self) {
        println!("urban record → {}", self.content)
    }
}
