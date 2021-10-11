use crate::cli::Opts;
use crate::db::Cache;

pub fn lookup(word: &str, dict_name: &str, db_cache: &Cache, opts: &Opts) {
    let (url, info): (String, Info) = match dict_name {
        "yahoo" => (yahoo::get_query_url(word), yahoo::INFO),
        _ => unreachable!(),
    };

    if opts.show_provider { println!("\x1b[34m[{}]\x1b[0m", info.provider); }
    if opts.show_url { println!("\x1b[34m({})\x1b[0m", url); }

    if let Some(content_string) = db_cache.query(word, info.name) {
        match dict_name {
            "yahoo" => yahoo::Content::from_str(content_string.as_str()).show(),
            _ => unreachable!(),
        };
    } else {
        match dict_name {
            "yahoo" => {
                let content = yahoo::query(word);
                db_cache.save(word, info.name, content.to_string().as_str());
                content.show()
            },
            _ => unreachable!(),
        };
    }
}

struct Info {
    provider: &'static str,
    name: &'static str,
}

trait Lookup<'a> {}

mod yahoo {
    use super::Info;
    pub(super) fn get_query_url(word: &str) -> String {
        format!("http://yahoo/{word}", word=word)
    }
    pub(super) const INFO: Info = Info {
        provider: "yahoo_provider",
        name: "yahoo_name",
    };
    pub(super) struct Content;
    impl Content {
        pub(super) fn show(&self) {
            log::info!("show content");
        }
        pub(super) fn from_str(_serialized: &str) -> Self {
            log::info!("deserialize content string");
            Content
        }
    }
    impl ToString for Content {
        fn to_string(&self) -> String {
            log::info!("serialize content to string");
            "serialized_self".to_string()
        }
    }
    pub(super) fn query(_word: &str) -> Content {
        log::info!("querying ...");
        Content
    }
}
