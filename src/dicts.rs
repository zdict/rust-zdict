use crate::cli::Opts;

macro_rules! register_dicts {
    ($($d:ident),+) => {
        $(
        mod $d;
        )+

        /// normal mode
        pub fn use_dict(mut opts: Opts) {
            let dict = opts.dict.unwrap_or("yahoo".to_string());
            opts.dict = None;

            let words = opts.words;
            opts.words = vec![];

            let use_db_cache: bool = false;

            for word /* String */ in words.into_iter() {
                match dict.as_str() {
                    $(
                    stringify!($d) => $d::Dict.lookup(word, use_db_cache, &opts),
                    )+
                    _ => unreachable!(),
                }
            }
        }

        pub fn list_dicts() {
            $(
            println!("{}: {}\n{}\n", $d::Dict::PROVIDER, $d::Dict::TITLE, $d::Dict::HOMEPAGE_URL);
            )+
        }
    };
}
register_dicts!(yahoo, urban);

trait Lookup {
    const HOMEPAGE_URL: &'static str ;
    const API: &'static str ;
    const TITLE: &'static str ;
    const PROVIDER: &'static str ;
    type Record: Display;
    fn query(&self, url: &str) -> Self::Record;

    fn get_url(&self, word: &str) -> String {
        Self::API.replace("{word}", word)
    }
    fn query_db_cache(&self, word: &str) -> Option<Self::Record> {
        println!("provider -> {}; word -> {}", Self::PROVIDER, word);
        None /* Record { .... } */
    }
    fn lookup(&self, word: String, use_db_cache: bool, opts: &Opts) {
        println!("opts => {:?}", opts);

        let url: String = self.get_url(&word);

        if opts.show_provider {
            println!("\x1b[34m[{}]\x1b[0m", Self::PROVIDER);
        }

        if opts.show_url {
            println!("\x1b[34m({})\x1b[0m", url);
        }

        if use_db_cache {
            if let Some(record) = self.query_db_cache(&word) {
                return record.show()
            }
        }
        self.query(&url).show()
    }
}

trait Display {
    fn show(&self) {}
}
