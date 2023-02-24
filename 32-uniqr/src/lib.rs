use clap::{ArgGroup, Parser};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    #[arg(help = "Input file", required = false, default_value = "-")]
    in_file: String,

    #[arg(help = "Output file")]
    out_file: Option<String>,

    #[arg(help = "Show counts", long, short = 'c', required = false)]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

// ---------------

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
