use html5ever::local_name;
use kuchiki::traits::*;
use serde::{Serialize, Deserialize};

use super::{Dict, Lookup, QueryResult, SerdeResult};


const VERSION: u8 = 3;


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Entry {
    version: u8,
    summary: Summary,
    explain: Vec<ExToken>,
    verbose: Vec<VerToken>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Summary {
    word: String,
    pronounce: Vec<(String, String)>,
    explain: Vec<SumExToken>,
    grammar: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum SumExToken {
    PoS(String),
    Explain(String),
}

#[derive(Serialize, Deserialize, Debug)]
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


impl Lookup for Entry {
    const DICT: Dict = Dict {
        name: "yahoo",
        title: "Yahoo Dictionary",
        homepage_url: "https://tw.dictionary.yahoo.com/",
    };

    fn get_query_url(word: &str) -> String {
        format!("https://tw.dictionary.search.yahoo.com/search?p={word}", word=word)
    }

    fn from_str(s: &str) -> SerdeResult<Self> { serde_json::from_str(s) }

    fn to_string(&self) -> SerdeResult<String> { serde_json::to_string(self) }

    fn query(content: QueryResult<String>) -> QueryResult<Option<Self>> {
        let document = kuchiki::parse_html().one(content?);

        let summary = {
            let summary = match document.select_first("div#web ol.searchCenterMiddle") {
                Err(_)=> return Ok(None),
                Ok(v) => v,
            };

            let nodes = match summary.as_node().select("div.sys_dict_word_card > div.grp-main > div") {
                Err(_) => return Ok(None),
                Ok(v) => v,
            };
            let nodes = nodes.collect::<Vec<_>>();
            let (word, pronounce, explain) = match nodes.as_slice() {
                // length 4..
                [word, pronounce, _, .., explain] => (word, Some(pronounce), explain),
                // length 2..4
                [word, .., explain] => (word, None, explain),
                // length 0..2
                _ => {
                    log::warn!("abnormal summary content, treat as not found");
                    return Ok(None);
                },
            };

            let word = word.as_node().text_contents().trim().to_string();

            let pronounce = {
                if let Some(p) = pronounce {
                    let mut ps = vec![];
                    for x in p.text_contents().split_whitespace() {
                        if let Some(i) = x.find('[') {
                            ps.push((x[..i].to_string(), x[i..].to_string()));
                        }
                    }
                    ps
                } else {
                    vec![]
                }
            };

            let explain = {
                if let Ok(ms) = explain.as_node().select("ul > li div") {
                    let mut tokens = vec![];
                    for m in ms {
                        if let Some(cls_attr) = m.attributes.borrow().get("class") {
                            if cls_attr.contains("pos_button") {
                                tokens.push(SumExToken::PoS(m.text_contents()));
                            } else if cls_attr.contains("dictionaryExplanation") {
                                tokens.push(SumExToken::Explain(m.text_contents()));
                            } else {}
                        }
                    }
                    tokens
                } else {
                    vec![]
                }
            };

            let grammar = {
                if let Ok(li) = summary.as_node().select("div.dictionaryWordCard > ul > li") {
                    li.map(|s| s.text_contents()).collect()
                } else {
                    vec![]
                }
            };

            Summary { word, pronounce, explain, grammar }
        };

        let explain = {
            let parse_item = |elm: kuchiki::NodeDataRef<kuchiki::ElementData>| {
                if let Ok(mut span_nodes) = elm.as_node().select("span") {
                    let text = format!("{} {}", span_nodes.next()?.text_contents(), span_nodes.next()?.text_contents());
                    let mut sentence = vec![];
                    for span in span_nodes {
                        for node in span.as_node().children() {
                            if node.as_element().is_some() {
                                sentence.push(SenToken::Bold(node.text_contents()));
                            } else {
                                sentence.push(SenToken::Plain(node.text_contents()));
                            }
                        }

                        if let Some(SenToken::Plain(s)) = sentence.pop() {
                            let (hd,tl) = s.rsplit_once(' ')?;
                            sentence.extend([hd,"\n",tl,"\n"].map(|s| SenToken::Plain(s.to_string())));
                        }
                    }
                    Some((text, sentence))
                } else {
                    None
                }
            };

            let to_tokens = |explain: kuchiki::NodeDataRef<kuchiki::ElementData>| -> Vec<ExToken> {
                let mut tokens = vec![];
                for elm in explain.as_node().children().elements() {
                    if let Some(cls_attr) = elm.attributes.borrow().get("class") {
                        if cls_attr.contains("compTitle") {
                            tokens.push(ExToken::PoS { text: elm.text_contents() })
                        } else if cls_attr.contains("compTextList") {
                            if let Ok(li) = elm.as_node().select("li") {
                                for item in li {
                                    if let Some((text, sentence)) = parse_item(item) {
                                        tokens.push(ExToken::Item { text, sentence });
                                    }
                                }
                            }
                        } else {}
                    }
                }
                tokens
            };

            if let Ok(explain) = document.select_first("div.tab-content-explanation") {
                to_tokens(explain)
            } else {
                vec![]
            }
        };

        let verbose = {
            if let Ok(synonyms) = document.select_first("div.tab-content-synonyms") {
                let mut verbose = vec![];
                for elm in synonyms.as_node().children().elements() {
                    match elm.name.local {
                        local_name!("div") => {
                            if let Ok(span) = elm.as_node().select_first(".fw-xl") {
                                verbose.push(VerToken::Title(span.text_contents()));
                            } else if let Ok(span) = elm.as_node().select_first(".fw-500") {
                                verbose.push(VerToken::Explain(span.text_contents()));
                            } else {}
                        },
                        local_name!("ul") => {
                            if let Ok(span_nodes) = elm.as_node().select("li > span") {
                                verbose.extend(span_nodes.map(|span|
                                    VerToken::Item(span.text_contents())
                                ));
                            }
                        },
                        _ => (),
                    }
                }
                verbose
            } else {
                vec![]
            }
        };

        Ok(Some(Entry { version: VERSION, summary, explain, verbose }))
    }

    fn show(&self, verbose: u8) {
        {
            let Summary { word, pronounce, explain, grammar } = &self.summary;
            println!("\x1b[33m{}\x1b[0m", word);
            if !pronounce.is_empty() {
                for (k, v) in pronounce {
                    print!("\x1b[0m{}\x1b[0m\x1b[37;1m{}\x1b[0m ", k, v);
                }
                println!();
            }
            for token in explain {
                match token {
                    SumExToken::PoS(v) => print!("  \x1b[31;1m{}\x1b[0m ", v),
                    SumExToken::Explain(v) => println!("\x1b[0m{}\x1b[0m", v),
                }
            }
            if !grammar.is_empty() {
                println!();
                for v in grammar {
                    println!("  \x1b[0m{}\x1b[0m", v);
                }
            }
        }

        if !self.explain.is_empty() {
            println!();
            for token in &self.explain {
                match token {
                    ExToken::PoS { text } => {
                        println!("\x1b[31;1m{}\x1b[0m", text);
                    },
                    ExToken::Item { text, sentence } => {
                        println!("  \x1b[0m{}\x1b[0m", text);
                        let mut is_line_start = true;
                        for s in sentence {
                            if is_line_start {
                                print!("    ");
                                is_line_start = false;
                            }
                            match s {
                                SenToken::Plain(s) if s != "\n" => {
                                    print!("\x1b[36m{}\x1b[0m", s);
                                },
                                SenToken::Bold(s) => {
                                    print!("\x1b[36;1m{}\x1b[0m", s);
                                },
                                SenToken::Plain(_) => {
                                    println!();
                                    is_line_start = true;
                                },
                            }
                        }
                    }
                }
            }
        }

        if verbose > 0 && !self.verbose.is_empty() {
            println!();
            for x in &self.verbose {
                match x {
                    VerToken::Title(v) => println!("\x1b[31;1m{}\x1b[0m", v),
                    VerToken::Explain(v) => println!("  \x1b[0m{}\x1b[0m", v),
                    VerToken::Item(v) => println!("    \x1b[36m{}\x1b[0m", v),
                }
            }
        }

        println!();
    }
}
