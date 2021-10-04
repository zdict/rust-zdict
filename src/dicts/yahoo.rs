use super::{Lookup, Display};
use reqwest;
use kuchiki::traits::*;
use kuchiki::{NodeRef, NodeDataRef, ElementData};
use html5ever::local_name;
use serde_json::{json, Value};

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
        let document = kuchiki::parse_html().one(content);

        Record {
            summary: parse_summary(&document),
            explain: parse_explain(&document),
            verbose: parse_verbose(&document),
        }
    }
}

fn parse_summary(_content: &NodeRef) -> Value {
    json!(null)
}

macro_rules! next_text { ($nodes:expr) => ($nodes.next().unwrap().text_contents()) }
fn parse_explain(document: &NodeRef) -> Value {
    let explanation = document.select_first("div.tab-content-explanation").unwrap();
    //println!("[DEBUG] explanation -> {:?}", explain);

    let parse_item = |elm: NodeDataRef<ElementData>| {
        let mut span_nodes = elm.as_node().select("span").unwrap();
        let text = format!("{} {}", next_text!(span_nodes), next_text!(span_nodes));
        let sentence: Value = span_nodes.map(|span| {
            let mut piece: Vec<_> = span.as_node().children().map(|node|
                if node.as_element().is_some() {
                    json!(["b", node.text_contents()])
                } else {
                    json!(node.text_contents())
                }
            ).collect();

            if let Some(Value::String(s)) = piece.pop() {
                let (hd,tl) = s.rsplit_once(' ').unwrap();
                piece.push(json!(hd));
                piece.push(json!("\n"));
                piece.push(json!(tl));
                piece.push(json!("\n"));
            }

            piece
        }).flatten().collect();

        //println!("[DEBUG] sentence -> {:?}", sentence);
        json!({"type": "item", "text": text, "sentence": sentence})
    };

    let elements = explanation.as_node().children().elements();
    let explain: Value = elements.map(|elm| {
        match elm.attributes.borrow().get("class").unwrap() {
            cls_attr if cls_attr.contains("compTitle") => {
                //println!("[DEBUG] cls attr => {:?}", cls_attr);
                let j = json!({"type": "PoS", "text": elm.text_contents()});
                //println!("[DEBUG] json => {:?}", j);
                Some(j)
            },
            cls_attr if cls_attr.contains("compTextList") => {
                //println!("[DEBUG] cls attr => {:?}", cls_attr);
                let ul: Value = elm.as_node().select("li").unwrap().map(parse_item).collect();
                //println!("[DEBUG] json => {:?}", ul);
                Some(ul)
            },
            _ => None,
        }
    }).flatten().collect();

    explain
}

fn parse_verbose(document: &NodeRef) -> Value {
    let synonyms = document.select_first("div.tab-content-synonyms").unwrap();
    let verbose = synonyms.as_node().children().elements().map(|elm| {
        match elm.name.local {
            local_name!("div") => {
                if let Ok(span) = elm.as_node().select_first(".fw-xl") {
                    Some(json!(["title", span.text_contents()]))
                } else if let Ok(span) = elm.as_node().select_first(".fw-500") {
                    Some(json!(["explain", span.text_contents()]))
                } else {
                    None
                }
            },
            local_name!("ul") => {
                Some(elm.as_node().select("li > span").unwrap().map(
                    |span| json!(["item", span.text_contents()])
                    ).collect())
            },
            _ => None,
        }
    }).flatten().collect();

    verbose
}

pub struct Record {
    summary: Value,
    explain: Value,
    verbose: Value,
}
impl Display for Record {
    fn show(&self) {
        println!("yahoo record → {:?}, {:?}, {:?}", self.summary, self.explain, self.verbose)
    }
}
