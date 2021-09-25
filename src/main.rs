#[allow(unused)] mod cli;
#[allow(unused)] mod yahoo;

fn main() {
    let _ = yahoo::main();
}

fn _main() {
    let opts = cli::parse_opts();
    let _mats= cli::get_matches!();

    if opts.subcmd.is_some() {
        /* any better way to output ? */
        println!("\
            yahoo: Yahoo Dictionary\n\
            https://tw.dictionary.yahoo.com/
        ");
    } else {
        /*
        yahoo = dicts::yahoo  // a constant of DictBase { ... }
        lookup = yahoo.lookup(&word)
        if show_provider:
            println!("{}", lookup.provider)  // blue
        if show_url:
            println!("{}", lookup.url)  // blue
        loop {
            if use_db_cache:
                if let Some(record) = lookup.query_db():
                    println!("{}", record)
                    break
            record = lookup.query().expect("error message")
            println!("{}", record)
            break
        }
        */
    }
}
