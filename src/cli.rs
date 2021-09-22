use clap::{AppSettings, Clap};

#[derive(Clap, Debug, PartialEq)]
#[clap(name = "zdict")]
//#[clap(version = env!("CARGO_PKG_VERSION"))]
//#[clap(settings = &[AppSettings::ColoredHelp])]  // not support yet
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
    words: Vec<String>,

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

#[derive(Clap, Debug, PartialEq)]
enum SubCommand {
    #[clap(name = "dicts", about = "Show currently supported dictionaries")]
    ListDicts,
}

pub fn get_parsed_opts() -> Opts { Opts::parse() }

/* ================================================== */

pub use clap::{App, load_yaml};

macro_rules! get_app_matches {
    () => { $crate::cli::App::from($crate::cli::load_yaml!("args.yaml")).get_matches() }
} pub(crate) use get_app_matches;

/* ================================================== */

#[cfg(test)]
mod opts_design {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => { vec![$($x.to_string()),*] };
    }

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
            verbose: 0,
            subcmd: Some(SubCommand::ListDicts),
        });
    }

    #[test]
    fn given_one_word() {
        let parsed_opts = Opts::try_parse_from(&["zd", "moe"]).unwrap();
        assert_eq!(parsed_opts, Opts {
            words: vec_of_strings!["moe"],
            show_provider: false,
            show_url: false,
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
mod yaml {
    //use super::*;

    #[test]
    fn list_dict() {
    }
}
