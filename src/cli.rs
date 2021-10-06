use clap::{AppSettings, Clap};

#[derive(Clap, Debug, PartialEq)]
#[clap(name = "zdict")]
//#[clap(version = env!("CARGO_PKG_VERSION"))]
//#[clap(settings = &[AppSettings::ColoredHelp])]  // not support `setting *s*` yet
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::DisableVersionForSubcommands)]
#[clap(setting = AppSettings::DisableHelpSubcommand)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]  // works fine unlike yaml version
#[clap(setting = AppSettings::ArgsNegateSubcommands)]
#[clap(setting = AppSettings::SubcommandsNegateReqs)]
pub struct Opts {
    #[clap(about = "Search translation of words")]
    #[clap(value_name = "word")]
    #[clap(required = true)]
    pub words: Vec<String>,

    #[clap(long, about = "Show dictionary provider")]
    pub show_provider: bool,

    #[clap(long, about = "Show URL")]
    pub show_url: bool,

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
    //dicts: Vec<String>,

    #[clap(long, about = "Choose the dictionary")]
    #[clap(possible_values = &["yahoo", "urban", "all"])]
    //#[clap(default_value = "yahoo")]  // avoid default value to break `ArgRequiredElseHelp`
    pub dict: Option<String>,

    #[clap(short, long, about = "Use verbose output")]
    #[clap(max_occurrences=2, parse(from_occurrences))]
    pub verbose: u8,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug, PartialEq)]
pub enum SubCommand {
    #[clap(name = "dicts", about = "Show currently supported dictionaries")]
    ListDicts,
}

pub fn parse_opts() -> Opts { Opts::parse() }

/* ================================================== */

pub use clap::{App, load_yaml};

#[allow(unused)] macro_rules! get_matches {
    () => { $crate::cli::App::from($crate::cli::load_yaml!("args.yaml")).get_matches() }
} #[allow(unused)] pub(crate) use get_matches;

/* ================================================== */

#[cfg(test)]
mod opts_design {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => { vec![$($x.to_string()),*] };
    } pub(super) use vec_of_strings;

    #[test]
    fn display_help_when_no_arguments() {
        let err = Opts::try_parse_from(&["zd"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    }

    #[test]
    fn list_dict() {
        let parsed_opts = Opts::try_parse_from(&["zd", "dicts"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec![],
            show_provider: false,
            show_url: false,
            dict: None,
            verbose: 0,
            subcmd: Some(SubCommand::ListDicts),
        });
    }

    #[test]
    fn require_words() {
        let err = Opts::try_parse_from(&["zd", "-v"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn given_one_word() {
        let parsed_opts = Opts::try_parse_from(&["zd", "moe"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe"],
            show_provider: false,
            show_url: false,
            dict: None,
            verbose: 0,
            subcmd: None,
        });
    }

    #[test]
    fn given_three_words() {
        let parsed_opts = Opts::try_parse_from(&["zd", "moe", "moe", "moe"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe", "moe", "moe"],
            show_provider: false,
            show_url: false,
            dict: None,
            verbose: 0,
            subcmd: None,
        });
    }

    #[test]
    fn set_flag_after_word() {
        let parsed_opts = Opts::try_parse_from(&["zd", "moe", "--show-provider"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe"],
            show_provider: true,
            show_url: false,
            dict: None,
            verbose: 0,
            subcmd: None,
        });
    }

    #[test]
    fn set_flag_before_word() {
        let parsed_opts = Opts::try_parse_from(&["zd", "--show-url", "moe"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe"],
            show_provider: false,
            show_url: true,
            dict: None,
            verbose: 0,
            subcmd: None,
        });
    }

    #[test]
    fn set_flag_before_word_as_subcommand() {
        let parsed_opts = Opts::try_parse_from(&["zd", "--show-url", "dicts"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["dicts"],
            show_provider: false,
            show_url: true,
            dict: None,
            verbose: 0,
            subcmd: None,
        });
    }

    #[test]
    fn enable_verbose() {
        let parsed_opts = Opts::try_parse_from(&["zd", "-v", "moe", "--verbose"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe"],
            show_provider: false,
            show_url: false,
            dict: None,
            verbose: 2,
            subcmd: None,
        });
    }

    #[test]
    fn reject_too_many_verbose() {
        let err = Opts::try_parse_from(&["zd", "-vv", "moe", "--verbose"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::TooManyOccurrences);
    }

    #[test]
    fn not_allow_arguments_after_subcommand() {
        let err = Opts::try_parse_from(&["zd", "dicts", "--verbose"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::UnknownArgument);
    }
}

#[cfg(test)]
mod yaml_design {
    use super::*;
    use super::opts_design::vec_of_strings;

    macro_rules! try_get_matches_from {
        ($cmd:expr) => {
            App::from(load_yaml!("args.yaml")).try_get_matches_from($cmd)
        }
    }

    #[test]
    #[ignore = "not supported"]
    fn display_help_when_no_arguments() {
        let err = try_get_matches_from!(&["zd"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    }

    #[test]
    fn list_dict() {
        let mats = try_get_matches_from!(&["zd", "dicts"]).unwrap();
        assert!(!mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(!mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_some());
    }

    #[test]
    fn require_words() {
        let err = try_get_matches_from!(&["zd", "-v"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn given_one_word() {
        let mats = try_get_matches_from!(&["zd", "moe"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(!mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["moe"]);
    }

    #[test]
    fn given_three_words() {
        let mats = try_get_matches_from!(&["zd", "moe", "moe", "moe"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(!mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["moe", "moe", "moe"]);
    }

    #[test]
    fn set_flag_after_word() {
        let mats = try_get_matches_from!(&["zd", "moe", "--show-provider"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(mats.is_present("show provider"));
        assert!(!mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["moe"]);
    }

    #[test]
    fn set_flag_before_word() {
        let mats = try_get_matches_from!(&["zd", "--show-url", "moe"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["moe"]);
    }

    #[test]
    fn set_flag_before_word_as_subcommand() {
        let mats = try_get_matches_from!(&["zd", "--show-url", "dicts"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(mats.is_present("show url"));
        assert!(!mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["dicts"]);
    }

    #[test]
    fn enable_verbose() {
        let mats = try_get_matches_from!(&["zd", "-v", "moe", "--verbose"]).unwrap();
        assert!(mats.is_present("words"));
        assert!(!mats.is_present("show provider"));
        assert!(!mats.is_present("show url"));
        assert!(mats.is_present("verbose"));
        assert!(mats.subcommand_matches("dicts").is_none());

        let words: Vec<&str> = mats.values_of("words").unwrap().collect();
        assert_eq!(words, vec_of_strings!["moe"]);

        assert_eq!(mats.occurrences_of("verbose"), 2);
    }

    #[test]
    #[ignore = "not supported"]
    fn reject_too_many_verbose() {
        let err = try_get_matches_from!(&["zd", "-vv", "moe", "--verbose"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::TooManyOccurrences);
    }

    #[test]
    fn not_allow_arguments_after_subcommand() {
        let err = try_get_matches_from!(&["zd", "dicts", "--verbose"]).unwrap_err();
        assert_eq!(err.kind, clap::ErrorKind::UnknownArgument);
    }
}
