use kuchiki::traits::*;
use serde::{Serialize, Deserialize};

use super::{Info, Lookup, InnerStruct};


pub(super) struct Dict;

impl Lookup for Dict {
    type Content = Content;

    const INFO: Info = Info {
        name: "yahoo",
        title: "Yahoo Dictionary",
        homepage_url: "https://tw.dictionary.yahoo.com/",
    };

    fn get_query_url(word: &str) -> String {
        format!("https://tw.dictionary.search.yahoo.com/search?p={word}", word=word)
    }

    fn query(url: &str) -> Self::Content {
        log::debug!("url: {}", url);
        log::info!("querying ...");

        let response = reqwest::blocking::get(url).unwrap();
        let content = response.text().unwrap();
        let document = kuchiki::parse_html().one(content);

        Content {
            summary: parse_summary(&document),
            explain: parse_explain(&document),
            verbose: parse_verbose(&document),
        }
    }
}

fn parse_summary(_document: &kuchiki::NodeRef) -> Summary { Summary }
fn parse_explain(_document: &kuchiki::NodeRef) -> Option<Vec<ExToken>> { None }
fn parse_verbose(_document: &kuchiki::NodeRef) -> Option<Vec<VerToken>> { None }


#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    summary: Summary,
    explain: Option<Vec<ExToken>>,
    verbose: Option<Vec<VerToken>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Summary;

#[derive(Serialize, Deserialize, Debug)]
struct ExToken;

#[derive(Serialize, Deserialize, Debug)]
struct VerToken;

impl InnerStruct for Content {
    fn from_json_str(serialized: &str) -> Self {
        log::info!("deserialize content string");
        serde_json::from_str(serialized).unwrap()
    }

    fn to_json_string(&self) -> String {
        let s = serde_json::to_string(self).unwrap();
        log::debug!("seralized content: {:?}", s);
        s
    }

    fn show(&self, _verbose: u8) {
        log::info!("show content");
        println!("{:?}", self);
    }
}
