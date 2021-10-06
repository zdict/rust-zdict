mod cli;
mod dicts;

fn main() {
    let opts = cli::parse_opts();

    if opts.subcmd.is_some() {
        dicts::list_dicts();
    } else {
        dicts::use_dict(opts);
    }
}
