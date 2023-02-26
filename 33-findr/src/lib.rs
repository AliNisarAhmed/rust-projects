use crate::EntryType::*;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(required = false, default_value = ".", action = ArgAction::Append, last = true)]
    paths: Vec<String>,

    #[arg(required = false, short = 'n', long = "name")]
    names: Vec<Regex>,

    #[arg(value_enum, short = 't', long = "type", value_parser = parse_entry_type, action = ArgAction::Append)]
    entry_types: Vec<EntryType>,
}

fn parse_entry_type(str: &str) -> Result<EntryType, String> {
    match str {
        "f" => Ok(File),
        "d" => Ok(Dir),
        "l" => Ok(Link),
        _ => Err(format!("[possible values: d, f, l]")),
    }
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
