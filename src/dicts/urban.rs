use super::{Lookup, Display};

pub struct Dict;
impl Lookup for Dict {
    const NAME: &'static str = "urban";
    const API: &'static str = "http://urban/{word}";
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
