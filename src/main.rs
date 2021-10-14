mod cli;
mod dict;
mod db;

fn main() {
    env_logger::init();
    let opts = cli::parse_opts();

    if opts.subcmd.is_some() {
        dict::list_dicts();
    } else {
        let db_cache = db::Cache::new(opts.disable_db_cache);
        dict::lookup_words(opts, db_cache);
    }
}
