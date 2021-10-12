use crate::cli::Opts;
use crate::db::Cache;

macro_rules! register_dict {
    ($($dict:ident),+) => {
        pub fn list_dicts() {
            $(
            let info = $dict::INFO;
            println!("{}: {}\n{}\n", info.name, info.title, info.homepage_url);
            )+
        }

        pub fn lookup(word: &str, dict_name: &str, db_cache: &Cache, opts: &Opts) {
            match dict_name {
                $( stringify!($dict) => $dict::lookup(word, db_cache, opts), )+
                _ => unreachable!(),
            };
        }
    };
} register_dict! { yahoo }

macro_rules! pub_fn_lookup {
    () => {
        use crate::cli::Opts;
        use crate::db::Cache;

        pub fn lookup(word: &str, db_cache: &Cache, opts: &Opts) {
            let url = get_query_url(word);
            let info = INFO;

            if opts.show_provider { println!("\x1b[34m[{}]\x1b[0m", info.name); }
            if opts.show_url { println!("\x1b[34m({})\x1b[0m", url); }

            if let Some(content_string) = db_cache.query(word, info.name) {
                Content::from_json_str(content_string.as_str()).show();
            } else {
                let content = query(url.as_str());
                db_cache.save(word, info.name, content.to_json_string().as_str());
                content.show()
            }
        }
    };
}

struct Info {
    name: &'static str,
    title: &'static str,
    homepage_url: &'static str,
}

mod yahoo {
    use super::Info;
    pub(super) const INFO: Info = Info {
        name: "yahoo",
        title: "Yahoo Dictionary",
        homepage_url: "https://...",
    };

    pub_fn_lookup!{}

    fn get_query_url(word: &str) -> String {
        format!("http://yahoo/{word}", word=word)
    }

    fn query(url: &str) -> Content {
        log::debug!("url: {}", url);
        log::info!("querying ...");
        Content
    }

    struct Content;
    impl Content {
        fn show(&self) {
            log::info!("show content");
        }
        fn from_json_str(_serialized: &str) -> Self {
            log::info!("deserialize content string");
            Content
        }
        fn to_json_string(&self) -> String {
            log::info!("serialize content to string");
            "serialized_self".to_string()
        }
    }
}
