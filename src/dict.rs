use crate::cli::Opts;
use crate::db::Cache;


macro_rules! register_dicts {
    ($($d:ident),+) => {
        $( mod $d; )+

        pub fn list_dicts() {
            $(
            let dict = $d::Entry::DICT;
            println!("{}: {}\n{}\n", dict.name, dict.title, dict.homepage_url);
            )+
        }

        pub fn lookup_words(opts: Opts, db_cache: Option<Cache>) {
            let dict_name = opts.dict.as_ref().map_or("yahoo", |n| n.as_str());
            let words = opts.words;
            let opts = Opts {
                dict: Default::default(),
                words: Default::default(),
                .. opts
            };
            match dict_name {
                $(
                stringify!($d) => {
                    for word in words {
                        lookup::<$d::Entry>(&word, &opts, db_cache.as_ref());
                    }
                },
                )+
                _ => unreachable!(),
            }
        }
    };
} register_dicts! { urban }


#[derive(Debug)]
struct Dict {
    name: &'static str,
    title: &'static str,
    homepage_url: &'static str,
}


type BoxError = Box<dyn std::error::Error>;
type QueryResult<T> = Result<T, BoxError>;
type SerdeResult<T> = serde_json::Result<T>;


trait Lookup {
    const DICT: Dict;

    fn get_query_url(word: &str) -> String;

    fn from_str(s: &str) -> SerdeResult<Self> where Self: Sized;

    fn to_string(&self) -> SerdeResult<String> where Self: Sized;

    fn query(url: &str) -> QueryResult<Option<Self>> where Self: Sized;

    fn show(&self, verbose: u8);
}


fn lookup<Entry: Lookup>(word: &str, opts: &Opts, db_cache: Option<&Cache>) {
    let url = Entry::get_query_url(word);

    if opts.show_provider {
        println!("\x1b[34m[{}]\x1b[0m", Entry::DICT.name);
    }
    if opts.show_url {
        println!("\x1b[34m({})\x1b[0m", url);
    }

    if opts.disable_db_cache {
        log::info!("bypass read cache");
    } else if let Some(cache) = db_cache {
        match cache.query(word, Entry::DICT.name) {
            Err(err) => log::error!("{}", err),
            Ok(None) => log::info!("cache not found"),
            Ok(Some(cached)) => {
                match Entry::from_str(cached.as_str()) {
                    Err(err) => {
                        log::warn!("{}", err);
                        log::debug!("cached entry string: {}", cached);
                    },
                    Ok(entry) => {
                        entry.show(opts.verbose);
                        return;
                    },
                }
            },
        }
    }

    log::info!("query ⇒ {}", url);
    match Entry::query(url.as_str()) {
        Err(err) => show_failure::<Entry>(err, word),
        Ok(None) => println!("\x1b[33m\"{}\" not found!\x1b[0m", word),
        Ok(Some(entry)) => {
            if let Ok(ser_entry) = entry.to_string() {
                if let Some(cache) = db_cache {
                    if let Err(err) = cache.save(word, Entry::DICT.name, &ser_entry) {
                        log::error!("{}", err);
                    }
                }
            }
            entry.show(opts.verbose);
        },
    };
}

fn show_failure<Entry: Lookup>(err: BoxError, word: &str) {
    let repo = "https://github.com/zdict/rust-zdict/";
    let issues = format!("{}issues", repo);
    println!("\
        {err:?}\n\
        {err}\n\
        ================================================================================\n\
        Entryionary: {} ({})\n\
        Word: '{}'\n\
        ================================================================================\n\
        \n\
        Houston, we got a problem 😢\n\
        Please report the error message above to {}\
    ", Entry::DICT.title, Entry::DICT.name, word, issues, err=err);
}