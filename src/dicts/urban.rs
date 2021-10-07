use super::{Lookup, Display};
use serde::{Serialize, Deserialize};
use reqwest;
use serde_json;

pub struct Dict;
impl Lookup for Dict {
    const HOMEPAGE_URL: &'static str = "https://www.urbandictionary.com/";
    const API: &'static str = "http://api.urbandictionary.com/v0/define?term={word}";
    const TITLE: &'static str = "Yahoo Dictionary";
    const PROVIDER: &'static str = "yahoo";
    type Content = Content;
    fn query(&self, url: &str) -> Self::Content {
        // TODO: handle `unwrap` below
        let response = reqwest::blocking::get(url).unwrap();
        let content = response.text().unwrap();
        let content: Content = serde_json::from_str(&content).unwrap(); 
        content
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    list: Vec<Def>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Def {
    word: String,
    definition: String,
    example: String,

    #[serde(flatten)]
    omitted: serde_json::Value,
}
impl Display for Content {
    fn show(&self, _verbose: u8) {
        let def = &self.list[0];
        println!("\x1b[33m{}\x1b[0m", def.word);
        println!("  \x1b[0m{}\x1b[0m", def.definition);
        for line in def.example.lines() {
            println!("  \x1b[36m{}\x1b[0m", line);
        }
        println!();
    }
}
