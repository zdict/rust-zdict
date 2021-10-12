struct Opts { words: Vec<String>, dict: String, show_url: bool, disable_db_cache: bool }
struct Record { content: String }
fn get_query_url(_word: &str, _api: &str) -> String { "http://???/{word}".to_string() }
mod db {
    use super::*;
    pub struct Cache;
    pub fn init() -> Cache { Cache }
    impl Cache {
        pub(super) fn query(&self, _word: &str, _dict_name: &str) -> Option<Record> { Some(Record { content: "content".to_string() }) }
        pub(super) fn save(&self, _record: Record) { }
    }
}
struct DictInfo<'a> { name: &'a str, api: &'a str }
mod urban {
    use super::*;
    pub(super) const INFO: DictInfo = DictInfo { name: "urban", api: "http://urban/{word}" };
    pub struct Content {}
    impl Content {
        pub fn deserialize(_content: &str) -> Self { Content {} }
        fn serialize(&self) -> String { "content".to_string() }
        pub fn show(&self) {}
    }
    pub(super) fn query() -> (Content, Record) { let c = Content {}; let s = c.serialize(); (c, Record { content: s }) }
}
mod yahoo {
    use super::*;
    pub(super) const INFO: DictInfo = DictInfo { name: "yahoo", api: "http://yahoo/{word}" };
    pub struct Content {}
    impl Content {
        pub fn deserialize(_content: &str) -> Self { Content {} }
        fn serialize(&self) -> String { "content".to_string() }
        pub fn show(&self) {}
    }
    pub(super) fn query() -> (Content, Record) { let c = Content {}; let s = c.serialize(); (c, Record { content: s }) }
}

pub fn main() {
    let opts = Opts {
        words: vec!["ground".to_string(), "pound".to_string()],
        dict: "yahoo".to_string(),
        show_url: true,
        disable_db_cache: false,
    };

    let db_cache = db::init();

    let info = match opts.dict.as_str() {
        "yahoo" => yahoo::INFO,
        "urban" => urban::INFO,
        _ => unreachable!(),
    };

    for word in opts.words {
        dbg!(&word);

        let query_url = get_query_url(&word, info.api);

        if opts.show_url {
            println!("{}", query_url);
        }

        let mut cache_hit = false;
        if !opts.disable_db_cache {
            if let Some(record) = db_cache.query(&word, info.name) {
                match opts.dict.as_str() {
                    "yahoo" => yahoo::Content::deserialize(&record.content).show(),
                    "urban" => urban::Content::deserialize(&record.content).show(),
                    _ => unreachable!(),
                };
                cache_hit = true;
            }
        }
        if !cache_hit {
            match opts.dict.as_str() {
                "yahoo" => { let (c,r) = yahoo::query(); db_cache.save(r); c.show() },
                "urban" => { let (c,r) = urban::query(); db_cache.save(r); c.show() },
                _ => unreachable!(),
            };
        }

    }
}
