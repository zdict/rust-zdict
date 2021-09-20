use clap::{load_yaml, App};
use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(
    name = "zdict",
    //version = env!("CARGO_PKG_VERSION"),
    //settings = &[AppSettings::ColoredHelp],  // not support yet
    setting = AppSettings::ColoredHelp,
    //setting = AppSettings::ArgRequiredElseHelp,  // no effect due to required args exit
    setting = AppSettings::DisableVersionForSubcommands,
    setting = AppSettings::DisableHelpSubcommand,
    setting = AppSettings::ArgsNegateSubcommands,  // no effect yet
    setting = AppSettings::SubcommandsNegateReqs,
)]
struct Opts {
    #[clap(about = "Search translation of words")]
    #[clap(required = true)]
    word: Vec<String>,
    #[clap(long, about = "Show dictionary provider")]
    show_provider: bool,
    #[clap(long, about = "Show URL")]
    show_url: bool,
    // broken: it consumes all following input until next flag
    // issue: https://github.com/clap-rs/clap/issues/1772
    //
    //#[clap(
    //    long,
    //    possible_values = &["all", "yahoo"],
    //    default_value = "yahoo",
    //    //multiple_occurrences = true,
    //    //parse(from_str),
    //)]
    //dict: Vec<String>,
    #[clap(short, long, about = "Use verbose output")]
    #[clap(max_occurrences=2, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}


#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "dicts", about = "Show currently supported dictionaries")]
    ListDicts,
}


fn main() {
    let opts = Opts::parse(); dbg!(opts); return;

    let yaml = load_yaml!("args.yaml");
    let app = App::from(yaml).version("3.0");
    let matches = app.get_matches();

    if matches.subcommand_matches("dicts").is_some() {
        println!("⇒ dicts");
        // :!cargo run -- dicts
        //
        // ⇒ dicts
    } else if let Some(words) = matches.values_of::<&str>("words"){
        println!("words ⇒ {:?}", words.collect::<Vec<_>>());

        println!("is present show provider → {:?}", matches.is_present("show provider"));
        println!("is present show url → {:?}", matches.is_present("show url"));

        let vs: Vec<&str> = matches.values_of("dicts").unwrap().collect();
        println!("dicts → {:?}", vs);

        println!("occurrences of verbose → {:?}", matches.occurrences_of("verbose"));

        println!("is present list dicts → {:?}", matches.is_present("list dicts"));
        //:!cargo run -- moe --dict all --dict all -vvv
        //   Compiling zd v0.1.0 (/Users/apua/thoughts/rust_learn/zd)
        //    Finished dev [unoptimized + debuginfo] target(s) in 1.07s
        //     Running `target/debug/zd moe --dict all --dict all -vvv`
        //words ⇒ ["moe"]
        //is present show provider → false
        //is present show url → false
        //dicts → ["all", "all"]
        //occurrences of verbose → 3
        //is present list dicts → false

    }

}
