use super::{Lookup, Display};
use reqwest;
use serde_json::{from_str, Value};

pub struct Dict;
impl Lookup for Dict {
    const HOMEPAGE_URL: &'static str = "https://www.urbandictionary.com/";
    const API: &'static str = "http://api.urbandictionary.com/v0/define?term={word}";
    const TITLE: &'static str = "Yahoo Dictionary";
    const PROVIDER: &'static str = "yahoo";
    type Record = Record;
    fn query(&self, url: &str) -> Self::Record {
        let response = reqwest::blocking::get(url).expect("...");
        let content = response.text().expect("...");
        //println!("[DEBUG] {:?}", content);

        let content: Value = from_str(&content).expect("...");
        Record { content }
    }
}

pub struct Record {
    content: Value,
}
impl Display for Record {
    fn show(&self, word: &str, _verbose: u8) {
        //println!("[DEBUG] urban record → {}", self.content)
        let list = &self.content["list"];
        //println!("[DEBUG] {}", data);

        println!("\x1b[33m{}\x1b[0m", word);

        #[allow(clippy::never_loop)]
        for data in list.as_array().unwrap_or(&vec![]).iter() {
            println!("  \x1b[0m{}\x1b[0m", data["definition"].as_str().unwrap_or(""));

            for line in data["example"].as_str().unwrap_or("").lines() {
                println!("  \x1b[36m{}\x1b[0m", line);
            }

            println!();
            break;
        }
    }
}
