use super::{Lookup, Display};
use reqwest;
use kuchiki::traits::*;

pub struct Dict;
impl Lookup for Dict {
    const HOMEPAGE_URL: &'static str = "https://tw.dictionary.yahoo.com/";
    //const API: &'static str = "https://tw.dictionary.yahoo.com/dictionary?p={word}";
    const API: &'static str = "https://tw.dictionary.search.yahoo.com/search?p={word}";
    const TITLE: &'static str = "Yahoo Dictionary";
    const PROVIDER: &'static str = "yahoo";
    type Record = Record;
    fn query(&self, url: &str) -> Self::Record {
        let response = reqwest::blocking::get(url).expect("...");
        let content = response.text().expect("...");

        let summary = parse_summary(&content);
        let explain = parse_explain(&content);
        let verbose = parse_verbose(&content);
        Record { summary, explain, verbose }
    }
}

fn parse_summary(_content: &str) {  }
fn parse_explain(_content: &str) {  }
fn parse_verbose(content: &str) -> Option<i32> {
    let document = kuchiki::parse_html().one(content);
    let synonyms = document.select_first("div.tab-content-synonyms").unwrap();

    use html5ever::local_name;
    //assert_eq!(local_name!("div"), node.name.local);

    //fn t<T> (_:&T) -> &str { std::any::type_name::<T>() }

    for elm in synonyms.as_node().children().elements() {
        match elm.name.local {
            local_name!("div") => {
                if let Ok(span) = elm.as_node().select_first(".fw-xl") {
                    println!("[TODO] title -> {}", span.text_contents());
                } else if let Ok(span) = elm.as_node().select_first(".fw-500") {
                    println!("[TODO] explain -> {}", span.text_contents());
                }
            },
            local_name!("ul") => {
                //println!("[DEBUG] ul -> {:?}", elm); println!();
                //let vs: Vec<String> = elm.as_node().select("li > span").unwrap().map(
                //    |span| span.text_contents()
                //    ).collect();
                //println!("[TODO] span collect -> {:?}", vs);
                for span in elm.as_node().select("li > span").unwrap() {
                    println!("[TODO] item -> {}", span.text_contents())
                };
            },
            _ => (),
        }
    }

    None
}

pub struct Record {
    summary: (),
    explain: (),
    verbose: Option<i32>,
}
impl Display for Record {
    fn show(&self) {
        println!("yahoo record → {:?}, {:?}, {:?}", self.summary, self.explain, self.verbose)
    }
}
