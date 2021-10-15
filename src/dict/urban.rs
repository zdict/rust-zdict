use serde::{Serialize, Deserialize};

use super::{Dict, Lookup, QueryResult, SerdeResult};


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Entry {
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


impl Lookup for Entry {
    const DICT: Dict = Dict {
        name: "urban",
        title: "Urban Dictionary",
        homepage_url: "https://www.urbandictionary.com/",
    };

    fn get_query_url(word: &str) -> String {
        format!("http://api.urbandictionary.com/v0/define?term={word}", word=word)
    }

    fn from_str(s: &str) -> SerdeResult<Self> { serde_json::from_str(s) }

    fn to_string(&self) -> SerdeResult<String> { serde_json::to_string(self) }

    fn query(url: &str) -> QueryResult<Option<Self>> {
        let response = reqwest::blocking::get(url)?;
        let entry_string = response.text()?;
        let entry: Self = serde_json::from_str(entry_string.as_str())?;

        log::debug!("parsed entry: {:?}", entry);
        if entry.list.is_empty() {
            return Ok(None);
        }
        Ok(Some(entry))
    }

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
