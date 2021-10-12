use crate::cli::Opts;
use crate::db::Cache;


macro_rules! register_dicts {
    ($($d:ident),+) => {
        $( mod $d; )+

        pub fn list_dicts() {
            $(
            let info = $d::Dict::INFO;
            println!("{}: {}\n{}\n", info.name, info.title, info.homepage_url);
            )+
        }

        pub fn lookup(word: &str, dict_name: &str, db_cache: &Cache, opts: &Opts) {
            match dict_name {
                $(
                stringify!($d) => $d::Dict::lookup(word, db_cache, opts),
                )+
                _ => unreachable!(),
            }
        }

    };
} register_dicts! { yahoo, urban }


#[derive(Debug)]
struct Info {
    name: &'static str,
    title: &'static str,
    homepage_url: &'static str,
}


trait Lookup {
    type Content: InnerStruct;

    const INFO: Info;

    fn get_query_url(word: &str) -> String;

    fn query(url: &str) -> Self::Content;

    fn lookup(word: &str, db_cache: &Cache, opts: &Opts) {
        let url = Self::get_query_url(word);
        let info = Self::INFO;

        if opts.show_provider {
            println!("\x1b[34m[{}]\x1b[0m", info.name);
        }
        if opts.show_url {
            println!("\x1b[34m({})\x1b[0m", url);
        }

        let content = {
            if let Some(content_string) = db_cache.query(word, info.name) {
                Self::Content::from_json_str(content_string.as_str())
            } else {
                let content = Self::query(url.as_str());
                db_cache.save(word, info.name, content.to_json_string().as_str());
                content
            }
        };
        content.show(opts.verbose);
    }
}


trait InnerStruct {
    fn from_json_str(serialized: &str) -> Self;

    fn to_json_string(&self) -> String;

    fn show(&self, verbose: u8);
}
