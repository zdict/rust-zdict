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
        println!("searching words: {:?}", opts.words);
    }
}
