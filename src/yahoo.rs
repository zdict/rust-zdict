//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;  // work as well?
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const HOMEPAGE_URL: &str = "https://tw.dictionary.yahoo.com/";
const API: &str = "https://tw.dictionary.yahoo.com/dictionary?p={word}";


fn get_url(word: &str) -> String { API.replace("{word}", word) }

fn fetch() -> Result<String> { Ok(reqwest::blocking::get(get_url("love"))?.text()?) }

pub fn main() -> Result<()> {

    //let content = fetch()?;
    let content = std::fs::read_to_string("y.html")?;
    //println!("{}", content);

    //  let parser = html5ever::parse_document(
    //      markup5ever_rcdom::RcDom::default(),
    //      Default::default());


    //  use html5ever::tendril::TendrilSink;
    //  //let dom = parser.from_utf8().read_from(&mut content.as_bytes());
    //  //let dom = parser.from_file("y.html").unwrap();
    //  let dom = parser.read_from(&mut content);

    //  //use std::borrow::Borrow;
    //  //let document = dom.document.borrow();
    //  //let html = document.children[0].borrow();
    //  //let body = html.children[1].borrow();
    //  //println!("{:?}", body);

    use kuchiki::traits::*;
    let document = kuchiki::parse_html().one(content);
    //for m in document.select("div.tab-content-synonyms").unwrap() {
    //    let node = m.as_node();
    //    println!("find node");
    //}
    let m = document.select_first("div.tab-content-synonyms").unwrap();
    println!("{:?}", m.text_contents());
    //println!("{:?}", document.as_text().unwrap().borrow());

    Ok(())
}
