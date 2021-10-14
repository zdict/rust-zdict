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

        pub fn lookup_words(opts: Opts, db_cache: Cache) {
            let (opts, db_cache) = (&opts, &db_cache);
            let dict_name = opts.dict.as_ref().map_or("yahoo", |n| n.as_str());
            match dict_name {
              $(
                stringify!($d) => {
                    for word in opts.words.as_slice() {
                        lookup::<$d::Entry>(word, db_cache, opts);
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


#[derive(Debug)]
enum QueryError {
    QueryFailure(String),
    InvalidPayload(String),
    NotFound,
}

impl From<reqwest::Error> for QueryError {
    fn from(e: reqwest::Error) -> Self {
        Self::QueryFailure(e.to_string())
    }
}

impl From<serde_json::Error> for QueryError {
    fn from(e: serde_json::Error) -> Self {
        Self::InvalidPayload(e.to_string())
    }
}


type QueryResult<T> = Result<T, QueryError>;
type SerdeResult<T> = serde_json::Result<T>;


trait Lookup {
    const DICT: Dict;

    fn get_query_url(word: &str) -> String;

    fn from_str(s: &str) -> SerdeResult<Self> where Self: Sized;

    fn to_string(&self) -> SerdeResult<String> where Self: Sized;

    fn query(url: &str) -> QueryResult<Self> where Self: Sized;

    fn show(&self, verbose: u8);
}


fn lookup<Entry: Lookup>(word: &str, db_cache: &Cache, opts: &Opts) {
    let url = Entry::get_query_url(word);

    if opts.show_provider {
        println!("\x1b[34m[{}]\x1b[0m", Entry::DICT.name);
    }
    if opts.show_url {
        println!("\x1b[34m({})\x1b[0m", url);
    }

    if opts.disable_db_cache {
        log::info!("bypass read cache");
    } else {
        //match Some("{\"list\":[{\"word\":\"ailin\",\"definition\":\"...\",\"example\":\"\"}]}".to_string()) {
        match db_cache.query(word, Entry::DICT.name) {
            None => log::info!("cache not found"),
            Some(cached) => {
                match Entry::from_str(cached.as_str()) {
                    Err(err) => {
                        log::warn!("unexpected fail to parse cached entry");
                        log::debug!("cached: {}", cached);
                        log::debug!("err: {:?}", err);
                    },
                    Ok(entry) => {
                        entry.show(opts.verbose);
                        return;
                    },
                }
            },
        }
    }

    // TODO: handle error and print/log out
    match Entry::query(url.as_str()) {
        Err(err) => show_failure::<Entry>(err, word),
        Ok(entry) => {
            //db_cache.save(word, Entry::DICT.name, entry.to_string().as_str());
            entry.show(opts.verbose);
        },
    }
}

fn show_failure<Entry: Lookup>(err: QueryError, word: &str) {
    log::debug!("{:?}", err);
    println!("{}", match err {
        QueryError::NotFound => format!("\x1b[33m\"{}\" not found!\x1b[0m", word),
        _ => format!("\
            ================================================================================\n\
            Entryionary: {} ({})\n\
            Word: '{}'\n\
            ================================================================================\n\
            \n\
            Houston, we got a problem 😢\n\
            Please report the error message above to https://github.com/zdict/zdict/issues\
        ", Entry::DICT.title, Entry::DICT.name, word),
    });
}
