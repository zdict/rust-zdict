use clap::{load_yaml, App};

fn main() {
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
