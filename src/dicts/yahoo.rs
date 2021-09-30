use super::{Lookup, Display};

pub struct Dict;
impl Lookup for Dict {
    const NAME: &'static str = "yahoo";
    const API: &'static str = "http://yahoo/{word}";
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
