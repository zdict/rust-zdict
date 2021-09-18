use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let app = App::from(yaml).version("3.0");
    app.get_matches();
}
