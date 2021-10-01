use super::{Lookup, Display};

pub struct Dict;
impl Lookup for Dict {
    const HOMEPAGE_URL: &'static str = "https://tw.dictionary.yahoo.com/";
    const API: &'static str = "https://tw.dictionary.yahoo.com/dictionary?p={word}";
    const TITLE: &'static str = "Yahoo Dictionary";
    const PROVIDER: &'static str = "yahoo";
    type Record = Record;
    fn query(&self, _url: &str) -> Self::Record {
        Record { summary: "summary", explain: "explain", verbose: "verbose" }
    }
}

pub struct Record {
    summary: &'static str,
    explain: &'static str,
    verbose: &'static str,
}
impl Display for Record {
    fn show(&self) {
        println!("yahoo record → {}, {}, {}", self.summary, self.explain, self.verbose)
    }
}
