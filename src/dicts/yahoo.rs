use super::{Lookup, Display};
use serde::{Serialize, Deserialize};
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
    type Content = Content;
    fn query(&self, url: &str) -> Self::Content {
        // TODO: handle `unwrap` below
        let response = reqwest::blocking::get(url).unwrap();
        let content = response.text().unwrap();
        let document = kuchiki::parse_html().one(content);

        Content {
            version: 2_u8,
            summary: parse_summary(&document),
            explain: parse_explain(&document),
            verbose: parse_verbose(&document),
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn parse_summary(document: &NodeRef) -> Value {
    let summary = document.select_first("div#web ol.searchCenterMiddle").unwrap();

    let grammar = summary.as_node().select("div.dictionaryWordCard > ul > li").unwrap();
    let grammar: Vec<_> = grammar.map(|s| s.text_contents()).collect();

    let nodes = summary.as_node().select("div.sys_dict_word_card > div.grp-main > div").unwrap();
    let nodes: Vec<_> = nodes.collect();

    let (w, p, e) = match nodes.as_slice() {
        [w, p, .., e] => (w, Some(p), e),
        [w, e] => (w, None, e),
        _ => unreachable!(),
    };

    let word = w.as_node().text_contents().trim().to_string();
    let pronounce = p.map(|p| {
        //let x = p.as_node().select_first("ul").unwrap().text_contents();
        let x = p.text_contents();
        let xs = x.split_whitespace();
        let xss: Vec<_> = xs.map(|x| {
            let i = x.find('[').unwrap();
            (x[..i].to_string(), x[i..].to_string())
        }).collect();
        xss
    });
    let explain = {
        let ms = e.as_node().select("ul > li div").unwrap();
        let ts: Vec<_> = ms.map(|m| {
            let k = match m.attributes.borrow().get("class").unwrap() {
                cls_attr if cls_attr.contains("pos_button") => "pos".to_string(),
                cls_attr if cls_attr.contains("dictionaryExplanation") => "explain".to_string(),
                _ => "?".to_string(),
            };
            (k, m.text_contents())
        }).collect();
        ts
    };

    let summary = json!({"word": word, "pronounce": pronounce, "explain": explain, "grammar": grammar});

    summary
}

macro_rules! next_text { ($nodes:expr) => ($nodes.next().unwrap().text_contents()) }
fn parse_explain(document: &NodeRef) -> Value {
    let explanation = document.select_first("div.tab-content-explanation").unwrap();

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

        json!({"type": "item", "text": text, "sentence": sentence})
    };

    let elements = explanation.as_node().children().elements();
    let explain: Value = elements.map(|elm| {
        match elm.attributes.borrow().get("class").unwrap() {
            cls_attr if cls_attr.contains("compTitle") => {
                let pos = json!({"type": "PoS", "text": elm.text_contents()});
                Some(vec![pos])
            },
            cls_attr if cls_attr.contains("compTextList") => {
                let ul: Vec<_> = elm.as_node().select("li").unwrap().map(parse_item).collect();
                Some(ul)
            },
            _ => None,
        }
    }).flatten().flatten().collect();

    explain
}

fn parse_verbose(document: &NodeRef) -> Option<Vec<VerToken>> {
    let synonyms = document.select_first("div.tab-content-synonyms");
    if synonyms.is_err() {
        return None;
    }

    let synonyms = synonyms.unwrap();
    let verbose = synonyms.as_node().children().elements().map(|elm| {
        match elm.name.local {
            local_name!("div") => {
                if let Ok(span) = elm.as_node().select_first(".fw-xl") {
                    Some(vec![VerToken::Title(span.text_contents())])
                } else if let Ok(span) = elm.as_node().select_first(".fw-500") {
                    Some(vec![VerToken::Explain(span.text_contents())])
                } else {
                    None
                }
            },
            local_name!("ul") => {
                Some(elm.as_node().select("li > span").unwrap().map(|span|
                    VerToken::Item(span.text_contents())
                ).collect())
            },
            _ => None,
        }
    }).flatten().flatten().collect();
    Some(verbose)
}

// TODO: #[serde(with = "module")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    version: u8,
    summary: Value, // Summary,
    explain: Value, // Vec<ExToken>,
    // TODO: serialize to [] if null
    verbose: Option<Vec<VerToken>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Summary {
    word: String,
    pronounce: Vec<[String;2]>,
    explain: Vec<SumExToken>,
    grammer: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
enum SumExToken {
    PoS(String),
    Explain(String),
}
#[derive(Serialize, Deserialize, Debug)]
// TODO: #[serde(tag = "type")]
enum ExToken {
    PoS { text: String },
    Item { text: String, sentence: Vec<SenToken> },
}
#[derive(Serialize, Deserialize, Debug)]
enum SenToken {
    Plain(String),
    Bold(String),
}
#[derive(Serialize, Deserialize, Debug)]
enum VerToken {
    Title(String),
    Explain(String),
    Item(String),
}
impl Display for Content {
    fn show(&self, verbose: u8) {
        // TODO: doesn't need `word` because summary has it
        show_summary(&self.summary);
        if !self.explain.as_array().unwrap().is_empty() {
            println!();
            show_explain(&self.explain);
        }
        if verbose > 0 && self.verbose.is_some() {
            println!();
            show_verbose(self.verbose.as_ref().unwrap());
        }
        println!();
    }
}

fn show_summary(summary: &Value) {
    let word = summary.get("word").unwrap();
    println!("\x1b[33m{}\x1b[0m", word.as_str().unwrap());

    let pronounce = summary.get("pronounce").unwrap().as_array().unwrap();
    for value in pronounce {
        let vs = value.as_array().unwrap();
        print!("\x1b[0m{}\x1b[0m", vs[0].as_str().unwrap());
        print!("\x1b[37;1m{}\x1b[0m ", vs[1].as_str().unwrap());
    }
    if !pronounce.is_empty() {
        println!();
    }

    let explain = summary.get("explain").unwrap().as_array().unwrap();
    for value in explain {
        let vs = value.as_array().unwrap();
        match vs[0].as_str().unwrap() {
            "pos" => print!("  \x1b[31;1m{}\x1b[0m ", vs[1].as_str().unwrap()),
            "explain" => println!("\x1b[0m{}\x1b[0m", vs[1].as_str().unwrap()),
            _ => unreachable!(),
        }
    }

    // XXX: grammar and pronounce are optional ?
    let grammar = summary.get("grammar").unwrap().as_array().unwrap();
    if !grammar.is_empty() {
        println!();
    }
    for value in grammar {
        println!("  \x1b[0m{}\x1b[0m", value.as_str().unwrap());
    }

}

fn show_explain(explain: &Value) {
    let show_sentence = |sentence: &Value| {
        let mut is_line_start = true;
        for s in sentence.as_array().unwrap().iter() {
            match s {
                Value::String(s) => {
                    if s == "\n" {
                        println!();
                        is_line_start = true;
                    } else {
                        if is_line_start {
                            print!("    \x1b[36m{}\x1b[0m", s);
                        } else {
                            print!("\x1b[36m{}\x1b[0m", s);
                        }
                        is_line_start = false;
                    }
                },
                Value::Array(s) => {
                    assert_eq!(s[0].as_str(), Some("b"));
                    let s = s[1].as_str().unwrap();
                    print!("\x1b[36;1m{}\x1b[0m", s);
                    is_line_start = false;
                },
                _ => unreachable!(),
            }
        }
    };

    for exp in explain.as_array().unwrap().iter() {
        let exp = exp.as_object().unwrap();
        match exp.get("type").unwrap().as_str().unwrap() {
            "PoS" => {
                // TODO: better structure, better way to get value than `unwrap` everywhere
                let s = exp.get("text").unwrap().as_str().unwrap();
                println!("\x1b[31;1m{}\x1b[0m", s);
            },
            "item" => {
                let s = exp.get("text").unwrap().as_str().unwrap();
                println!("  \x1b[0m{}\x1b[0m", s);
                if let Some(sentence) = exp.get("sentence") {
                    show_sentence(sentence);
                }
            }
            _ => unreachable!(),
        }
    }
}

fn show_verbose(verbose: &[VerToken]) {
    for x in verbose.iter() {
        match x {
            VerToken::Title(v) => println!("\x1b[31;1m{}\x1b[0m", v),
            VerToken::Explain(v) => println!("  \x1b[0m{}\x1b[0m", v),
            VerToken::Item(v) => println!("    \x1b[36m{}\x1b[0m", v),
        }
    }
}
