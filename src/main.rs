use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let app = App::from(yaml).version("3.0");
    let matches = app.get_matches();

    let vs: Vec<&str> = matches.values_of("words").unwrap().collect();
    println!("words → {:?}", vs);

    println!("is present show provider → {:?}", matches.is_present("show provider"));
    println!("is present show url → {:?}", matches.is_present("show url"));

    let vs: Vec<&str> = matches.values_of("dict").unwrap().collect();
    println!("dicts → {:?}", vs);

    println!("is present list dicts → {:?}", matches.is_present("list dicts"));
    println!("occurrences of verbose → {:?}", matches.occurrences_of("verbose"));

    // cargo run -- --list-dicts --dict yahoo --dict all -vvv moe moe moe
    //
    // words → ["moe", "moe", "moe"]
    // is present show provider → false
    // is present show url → false
    // dicts → ["yahoo", "all"]
    // is present list dicts → true
    // occurrences of verbose → 3
}
