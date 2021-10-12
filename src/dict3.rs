use crate::cli::Opts;
use crate::db::Cache;

macro_rules! register_dicts {
    ($($d:ident),+) => {
        pub fn list_dicts() {
            $(
            let info = $d::Dict.info();
            println!("{}: {}\n{}\n", info.name, info.title, info.homepage_url);
            )+
        }
        pub fn lookup(word: &str, dict_name: &str, db_cache: &Cache, opts: &Opts) {
            let dict: Box<dyn Lookup> = match dict_name {
                $( stringify!($d) => Box::new($d::Dict),)+
                _ => unreachable!(),
            };
            dict.lookup(word, db_cache, opts);
        }

    };
}
register_dicts! { yahoo }

struct Content;

#[derive(Debug)]
struct Info {
    name: &'static str,
    title: &'static str,
    homepage_url: &'static str,
}

trait Lookup {
    fn get_query_url(&self, word: &str) -> String;

    fn info(&self) -> Info;

    fn query(&self, url: &str) -> Content;

    fn from_json_str(&self, serialized: &str) -> Content;

    fn to_json_string(&self, content: &Content) -> String;

    fn show(&self, content: &Content);

    fn lookup(&self, word: &str, db_cache: &Cache, opts: &Opts) {
        let url = self.get_query_url(word);
        let info = self.info();

        if opts.show_provider {
            println!("\x1b[34m[{}]\x1b[0m", info.name);
        }
        if opts.show_url {
            println!("\x1b[34m({})\x1b[0m", url);
        }

        if let Some(content_string) = db_cache.query(word, info.name) {
            self.show(&self.from_json_str(content_string.as_str()));
        } else {
            let content = &self.query(url.as_str());
            db_cache.save(word, info.name, self.to_json_string(content).as_str());
            self.show(content);
        }
    }
}

mod yahoo {
    use super::{Info, Lookup, Content};

    pub(super) struct Dict;

    const INFO: Info = Info {
        name: "yahoo",
        title: "Yahoo Dictionary",
        homepage_url: "https://...",
    };

    impl Lookup for Dict {
        fn get_query_url(&self, word: &str) -> String {
            format!("http://yahoo/{word}", word=word)
        }
        fn info(&self) -> Info {
            INFO
        }
        fn query(&self, url: &str) -> Content {
            log::debug!("url: {}", url);
            log::info!("querying ...");
            Content
        }
        fn from_json_str(&self, _serialized: &str) -> Content {
            log::info!("deserialize content string");
            Content
        }
        fn to_json_string(&self, _content: &Content) -> String {
            log::info!("serialize content to string");
            "serialized_self".to_string()
        }
        fn show(&self, _content: &Content) {
            log::info!("show content");
        }
    }
}
