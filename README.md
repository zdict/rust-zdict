# zdict

A Rust version of [zdict](https://github.com/zdict/zdict).


## Install

```console
cargo install --git https://github.com/zdict/rust-zdict
```


## Usage

```
zdict

USAGE:
    zd [FLAGS] [OPTIONS] <word>...
    zd <SUBCOMMAND>

ARGS:
    <word>...    Search translation of words

FLAGS:
    -d, --disable-db-cache    Temporarily not using the result from db cache. (still save the result
                              into db)
    -h, --help                Print help information
        --show-provider       Show dictionary provider
        --show-url            Show URL
    -v, --verbose             Use verbose output
    -V, --version             Print version information

OPTIONS:
        --dict <DICT>    Choose the dictionary [possible values: yahoo, urban, all]

SUBCOMMANDS:
    dicts    Show currently supported dictionaries
```

![](https://user-images.githubusercontent.com/829295/137491540-e1db8ba4-0129-4d6e-af82-21b5399cfbb4.png)
