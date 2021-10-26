use serde::{Serialize, Deserialize};

use super::{Dict, Lookup, QueryResult, SerdeResult};


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Entry {
    data: Vec<Item>,
    #[serde(flatten)]
    omitted: serde_json::Value,  // eg: meta
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    japanese: Vec<Japenese>,
    senses: Vec<Sense>,

    #[serde(flatten)]
    omitted: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Japenese {
    reading: Option<String>,
    word: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sense {
    parts_of_speech: Vec<String>,
    english_definitions: Vec<String>,
    see_also: Vec<String>,
    restrictions: Vec<String>,
}

impl Lookup for Entry {
    const DICT: Dict = Dict {
        name: "jisho",
        title: "Jisho",
        homepage_url: "https://jisho.org/",
    };

    fn get_query_url(word: &str) -> String {
        format!("https://jisho.org/api/v1/search/words?keyword={word}", word=word)
    }

    fn from_str(s: &str) -> SerdeResult<Self> { serde_json::from_str(s) }

    fn to_string(&self) -> SerdeResult<String> { serde_json::to_string(self) }

    fn query(entry_string: QueryResult<String>) -> QueryResult<Option<Self>> {
        let entry: Entry = serde_json::from_str(entry_string?.as_str())?;
        if entry.data.is_empty() {
            return Ok(None);
        }
        Ok(Some(entry))
    }

    fn show(&self, verbose: u8) {
        for (i, item) in self.data.iter().enumerate() {
            if i > 1 {
                println!();
            }
            if let Some(reading) = &item.japanese[0].reading {
                println!("\x1b[33;1m{}\x1b[0m", reading);
            }
            if let Some(word) = &item.japanese[0].word {
                println!("\x1b[32;1m{}\x1b[0m", word);
            }
            for (idx, sense) in item.senses.iter().enumerate() {
                if !sense.parts_of_speech.is_empty() {
                    println!("\x1b[31;1m{}\x1b[0m", sense.parts_of_speech.join(", "));
                }
                println!("  \x1b[32;1m{}. {}\x1b[0m", idx+1, sense.english_definitions.join("; "));
                if !sense.see_also.is_empty() {
                    println!("    \x1b[34mSee also {}\x1b[0m", sense.see_also.join(", "));
                }
                if !sense.restrictions.is_empty() {
                    println!("    \x1b[34mOnly to {}\x1b[0m", sense.restrictions.join(", "));
                }
            }
            if item.japanese.len() > 1 {
                println!("\x1b[0mOther forms\x1b[0m");
                println!("  \x1b[33m{}\x1b[0m", item.japanese.iter().skip(1).map(|wf| { // word form
                    let reading = if let Some(reading) = &wf.reading { reading } else { "" };
                    let word = if let Some(word) = &wf.word { word } else { "" };
                    format!("{}[{}]", word, reading)
                }).collect::<Vec<_>>().join(", "));
            }

            if verbose == 0 {
                break;
            }
        }
        println!();
    }
}

