use clap::{AppSettings, Parser};


#[derive(clap::Parser, Debug, PartialEq, Default)]
#[clap(name = "zdict")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
//#[clap(settings = &[AppSettings::ColoredHelp])]  // not support `setting *s*` yet
//#[clap(setting = AppSettings::ColoredHelp)]  // not support in beta.5
//#[clap(setting = AppSettings::DisableVersionForSubcommands)]  // not support in beta.5
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
    #[clap(possible_values = crate::dict::DICTS)]
    //#[clap(default_value = "yahoo")]  // avoid default value to break `ArgRequiredElseHelp`
    pub dict: Option<String>,

    #[clap(short, long, about = "Use verbose output")]
    #[clap(max_occurrences=2, parse(from_occurrences))]
    pub verbose: u8,

    #[clap(short, long, about = "Temporarily not using the result from db cache. (still save the result into db)")]
    pub disable_db_cache: bool,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Parser, Debug, PartialEq)]
pub enum SubCommand {
    #[clap(name = "dicts", about = "Show currently supported dictionaries")]
    ListDicts,
}

pub fn parse_opts() -> Opts { Opts::parse() }
