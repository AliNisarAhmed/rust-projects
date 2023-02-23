use clap::{ArgGroup, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

    #[arg(help = "print number of lines", short = 'l', long)]
    lines: bool,

    #[arg(help = "print number of words", short = 'w', long)]
    words: bool,

    #[arg(help = "print number of bytes", short = 'c', long)]
    bytes: bool,

    #[arg(help = "print number of characters", short = 'm', long)]
    chars: bool,
}

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    pub fn add(&mut self, other: &FileInfo) {
        self.num_lines += other.num_lines;
        self.num_words += other.num_words;
        self.num_bytes += other.num_bytes;
        self.num_chars += other.num_chars;
    }
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    let mut lines = config.lines;
    let mut words = config.words;
    let mut bytes = config.bytes;
    let chars = config.chars;

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true
    }

    Ok(Config {
        lines,
        words,
        bytes,
        chars: config.chars,
        files: config.files,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total: FileInfo = Default::default();

    for filename in &config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let file_info = count(file).unwrap();

                total.add(&file_info);

                let file_info_string = gen_file_info_string(filename, file_info, &config);
                println!("{}", file_info_string);
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}",
            gen_file_info_string(&String::from("total"), total, &config)
        );
    }

    Ok(())
}

// ---------------------------------------------------

fn gen_file_info_string(filename: &String, file_info: FileInfo, config: &Config) -> String {
    let mut result = String::new();

    if config.lines {
        result.push_str(format_field(file_info.num_lines).as_str())
    }

    if config.words {
        result.push_str(format_field(file_info.num_words).as_str())
    }

    if config.bytes {
        result.push_str(format_field(file_info.num_bytes).as_str())
    }

    if config.chars {
        result.push_str(format_field(file_info.num_chars).as_str())
    }

    if filename != "-" {
        result.push_str(&format!(" {}", filename));
    }

    result
}

fn format_field(num: usize) -> String {
    format!("{:>8}", num)
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut file_info: FileInfo = Default::default();

    let mut line = String::new();

    while let Ok(n) = file.read_line(&mut line) {
        if n <= 0 {
            break;
        }

        file_info.num_lines += 1;
        file_info.num_words += line.split_whitespace().count();
        file_info.num_bytes += n;
        file_info.num_chars += line.chars().count();
        line.clear();
    }

    Ok(file_info)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)] // enables compilation only during testing
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
