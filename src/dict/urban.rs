use serde::{Serialize, Deserialize};

use super::{Info, Lookup, InnerStruct};


pub(super) struct Dict;

impl Lookup for Dict {
    type Content = Content;

    const INFO: Info = Info {
        name: "urban",
        title: "Urban Dictionary",
        homepage_url: "https://www.urbandictionary.com/",
    };

    fn get_query_url(word: &str) -> String {
        format!("http://api.urbandictionary.com/v0/define?term={word}", word=word)
    }

    fn query(url: &str) -> Self::Content {
        log::debug!("url: {}", url);
        log::info!("querying ...");

        let response = reqwest::blocking::get(url).unwrap();
        let content = &response.text().unwrap();

        serde_json::from_str(content).unwrap()
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

impl InnerStruct for Content {
    fn from_json_str(serialized: &str) -> Self {
        log::info!("deserialize content string");
        serde_json::from_str(serialized).unwrap()
    }

    fn to_json_string(&self) -> String {
        log::info!("serialize content to string");
        serde_json::to_string(self).unwrap()
    }

    fn show(&self, _verbose: u8) {
        log::info!("show content");
        let def = &self.list[0];
        println!("\x1b[33m{}\x1b[0m", def.word);
        println!("  \x1b[0m{}\x1b[0m", def.definition);
        for line in def.example.lines() {
            println!("  \x1b[36m{}\x1b[0m", line);
        }
        println!();
    }
}
