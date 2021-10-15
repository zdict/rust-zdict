mod cli;
mod dict;
mod db;

fn main() {
    env_logger::init();

    let opts = cli::parse_opts();

    if opts.subcmd.is_some() {
        dict::list_dicts();
    } else {
        dict::lookup_words(opts, db::Cache::new());
    }
}
