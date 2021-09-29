//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;  // work as well?
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TITLE: &str = "Yahoo Dictionary";
const PROVIDER: &str = "yahoo";

const HOMEPAGE_URL: &str = "https://tw.dictionary.yahoo.com/";
//const API: &str = "https://tw.dictionary.yahoo.com/dictionary?p={word}";
const API: &str = "https://tw.dictionary.search.yahoo.com/search?p={word}";

fn get_url(word: &str) -> String { API.replace("{word}", word) }

// `get_raw` might be overwritten by submodule
pub fn get_raw(word: &str) -> Result<String> {
    Ok(reqwest::blocking::get(get_url(word))?.text()?)
}

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

struct Record {
    version: u8,
    summary: Option<Summary>,
    explain: Option<Explain>,
    verbose: Verbose,
}

impl Record {
    fn show(&self) {}
    fn parse(content: &str) -> Self {
        Record {
            version: VERSION,
            summary: None,
            explain: None,
            verbose: Verbose(vec![]),
        }
    }
}

const VERSION: u8 = 2;

struct Summary {
    // eg: "love"
    word: String,
    // eg: [['KK', '[lʌv]'], ['DJ', '[lʌv]']]
    pronounce: Option<Vec<Vec<String>>>,
    // eg:
    // ['pos', 'n.'],
    // ['explain', '愛；熱愛；愛戴[U][（+for/of/to/fowards）]；戀愛，愛情[U][（+for）]'],
    // ['pos', 'vt.'], ['explain', '愛，熱愛；愛戴；疼愛；愛好，喜歡[+v-ing][+to-v]'],
    // ['pos', 'vi.'], ['explain', '愛[W]']],
    explain: Vec<String>,
    // eg:
    // ['名詞複數：loves',
    //   '過去式：loved   過去分詞：loved   現在分詞：loving'],
    grammar: Vec<String>,
}

// Is it a good way to do it??
impl Summary { fn parse(content: &str, word: &str) -> Self {
    Summary {
        word: "love".to_string(),
        pronounce: None,
        explain: vec!["...".to_string()],
        grammar: vec!["...".to_string()],
    }
}}

struct Explain;

fn explain(content: &str, word: &str) { }

//  {'explain': [
//      {'text': 'n.名詞', 'type': 'PoS'},
//          'type': 'item'},
//          'text': '1. 愛；熱愛；愛戴[U][（+for/of/to/fowards）]',
//          {'sentence':
//              ["My mother's ", ['b', 'love'], ' for me was very great.', '\n',
//              '我母親對我的愛是很深的。', '\n'],
//          'type': 'item'},
//          'text': '2. 戀愛，愛情[U][（+for）]',
//          {'sentence':
//              ['John and Mary are in ', ['b', 'love'], '.', '\n',
//              '約翰和瑪莉在相愛。', '\n'],
//          'type': 'item'},
//          'text': '5. （多指女性）情人[C]',
//          {'sentence': [],
//
//      {'text': 'vi.不及物動詞', 'type': 'PoS'},
//          {'sentence': [], 'text': '1. 愛[W]', 'type': 'item'}],

struct Verbose(Vec<[String;2]>);

//   'verbose': [
//       ['title', '同義詞'],
//          ['explain', 'n. 愛；喜好'],
//              ['item', 'affection'], ['item', 'devotion'], ['item', 'adoration'],
//          ['explain', 'vt. 愛，熱愛；喜愛'],
//              ['item', 'adore'], ['item', 'admire'], ['item', 'enjoy'],
//       ['title', '反義詞'],
//          ['explain', '「vi. & vt. 愛；疼愛；喜歡」的反義字'],
//              ['item', 'hate'], ['item', 'abhor'], ['item', 'detest'],
//          ['explain', '「n. 愛；疼愛」的反義字'],
//              ['item', 'hatred'], ['item', 'enmity'], ['item', 'spite']],
