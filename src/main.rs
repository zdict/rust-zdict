mod cli;
mod dict3;
use dict3 as dicts;
#[allow(dead_code, unused_variables)] mod db;

fn main() {
    env_logger::init();
    let opts = cli::parse_opts();

    if opts.subcmd.is_some() {
        //dicts::list_dicts();
    } else {
        let words = opts.words;
        let db_cache = db::Cache::new(opts.disable_db_cache);
        let dict_name = opts.dict.as_ref().map_or("yahoo", |n| n.as_str());
        let opts = cli::Opts {
            words: Default::default(),
            dict: Default::default(),
            .. opts
        };
        for word in words.as_slice() {
            log::info!("lookup word {:?} by dict named {:?}", word, dict_name);
            log::debug!("options: {:?}", &opts);
            dicts::lookup(word, dict_name, &db_cache, &opts);
        }
    }
}
