use crate::cli::Opts;

macro_rules! register_dicts {
    ($($d:ident),+) => {
        $(
        mod $d;
        )+

        pub fn lookup(opts: Opts) {
            let dict_name: String = if let Some(dict) = opts.dict { dict } else { "yahoo".to_string() };
            let use_db_cache: bool = false;
            for word /* String */ in opts.words.into_iter() {
                if opts.show_provider { }

                if opts.show_url { }

                match dict_name.as_str() {
                    $(
                    stringify!($d) => {
                        if use_db_cache {
                            if let Some(record /* <Self as Lookup>::Record */) = &$d::Dict.query_db_cache(&word) {
                                record.show()
                            } else {
                                $d::Dict.query(&word).show()
                            }
                        } else {
                            $d::Dict.query(&word).show()
                        }
                    },
                    )+
                    _ => unreachable!(),
                };
            };
        }

        pub fn list_dicts() {
            $(
            println!("{} {}\n", $d::Dict::NAME, $d::Dict::API);
            )+
        }
    };
}
register_dicts!(yahoo, urban);

trait Lookup {
    const NAME: &'static str;
    const API: &'static str;
    type Record: Display;
    fn query(&self, url: &str) -> Self::Record;

    fn get_url(&self, word: &str) -> String {
        Self::API.replace("{word}", word)
    }
    fn query_db_cache(&self, word: &str) -> Option<Self::Record> {
        println!("provider -> {}; word -> {}", Self::NAME, word);
        None /* Record { .... } */
    }
}

trait Display {
    fn show(&self) {}
}
