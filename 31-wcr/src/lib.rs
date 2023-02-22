use clap::builder::ArgPredicate;
use clap::{ArgGroup, Parser};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
        ArgGroup::new("exclusive_flags")
            .required(false)
            .multiple(false)
            .args(["bytes", "chars"])
        ))]
pub struct Config {
    #[arg(help = "Input file(s)", required = false, default_value = "-")]
    files: Vec<String>,

    #[arg(
        help = "print number of lines",
        short = 'l',
        default_value_if("words", ArgPredicate::IsPresent, "false"),
        default_value_if("bytes", ArgPredicate::IsPresent, "false")
    )]
    lines: bool,

    #[arg(
        help = "print number of words",
        short = 'w',
        default_value_if("lines", ArgPredicate::IsPresent, "false"),
        default_value_if("bytes", ArgPredicate::IsPresent, "false")
    )]
    words: bool,

    #[arg(
        help = "print number of bytes",
        short = 'c',
        default_value_if("words", ArgPredicate::IsPresent, "false"),
        default_value_if("lines", ArgPredicate::IsPresent, "false")
    )]
    bytes: bool,

    #[arg(help = "print number of characters", short = 'm')]
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
