use crate::EntryType::*;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(required = false, default_value = ".", action = ArgAction::Append)]
    paths: Vec<String>,

    #[arg(required = false, short = 'n', long = "name", value_parser = parse_name, num_args(1..))]
    names: Vec<Regex>,

    #[arg(value_enum, short = 't', long = "type", value_parser = parse_entry_type, num_args(1..))]
    entry_types: Vec<EntryType>,
}

fn parse_name(name: &str) -> Result<Regex, String> {
    Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name))
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
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    File => entry.file_type().is_file(),
                    Dir => entry.file_type().is_dir(),
                    Link => entry.file_type().is_symlink(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
