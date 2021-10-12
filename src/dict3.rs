use crate::cli::Opts;
use crate::db::Cache;

pub fn lookup(word: &str, dict_name: &str, db_cache: &Cache, opts: &Opts) {
    let (url, info): (String, Info) = match dict_name {
        "yahoo" => (yahoo::get_query_url(word), yahoo::INFO),
        _ => unreachable!(),
    };

    if opts.show_provider { println!("\x1b[34m[{}]\x1b[0m", info.name); }
    if opts.show_url { println!("\x1b[34m({})\x1b[0m", url); }

    if let Some(content_string) = db_cache.query(word, info.name) {
        match dict_name {
            "yahoo" => yahoo::Content::from_json_str(content_string.as_str()).show(),
            _ => unreachable!(),
        };
    } else {
        match dict_name {
            "yahoo" => {
                let content = yahoo::query(url.as_str());
                db_cache.save(word, info.name, content.to_json_string().as_str());
                content.show()
            },
            _ => unreachable!(),
        };
    }
}

struct Info {
    name: &'static str,
    title: &'static str,
    homepage_url: &'static str,
}

pub fn list_dicts() {
    let info = yahoo::INFO;
    println!("{}: {}\n{}\n", info.name, info.title, info.homepage_url);
}

mod yahoo {
    use super::Info;
    pub(super) fn get_query_url(word: &str) -> String {
        format!("http://yahoo/{word}", word=word)
    }
    pub(super) const INFO: Info = Info {
        name: "yahoo",
        title: "Yahoo Dictionary",
        homepage_url: "https://...",
    };
    pub(super) struct Content;
    impl Content {
        pub(super) fn show(&self) {
            log::info!("show content");
        }
        pub(super) fn from_json_str(_serialized: &str) -> Self {
            log::info!("deserialize content string");
            Content
        }
        pub(super) fn to_json_string(&self) -> String {
            log::info!("serialize content to string");
            "serialized_self".to_string()
        }
    }
    pub(super) fn query(url: &str) -> Content {
        log::debug!("url: {}", url);
        log::info!("querying ...");
        Content
    }
}
