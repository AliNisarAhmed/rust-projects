use clap::{ArgGroup, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(name = "catr")]
#[command(author = "Ali Ahmed")]
#[command(version = "1.0")]
#[command(about = "Rust cat", long_about = None)]
#[command(group(
        ArgGroup::new("flags")
            .required(false)
            .multiple(false)
            .args(["number", "number_nonblank"])
        ))]
pub struct Config {
    #[arg(help = "Input File(s)", required = false, default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long)]
    number: bool,

    #[arg(short = 'b', long)]
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut blanks = 0;

                for (index, line) in file.lines().enumerate() {
                    let line = line.unwrap();

                    let line_is_empty = line.is_empty();

                    let prefix = if config.number {
                        format!("{:>6}\t", index + 1)
                    } else if config.number_nonblank && !line_is_empty {
                        format!("{:>6}\t", index + 1 - blanks)
                    } else {
                        String::from("")
                    };

                    println!("{}{}", prefix, line);

                    if line_is_empty {
                        blanks += 1
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
