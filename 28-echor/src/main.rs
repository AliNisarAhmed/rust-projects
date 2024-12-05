use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor")]
#[command(author = "Ali Ahmed")]
#[command(version = "1.0")]
#[command(about = "Rust echo", long_about = None)]
struct Args {
    #[arg(help = "Input text", required = true, value_name = "TEXT")]
    text: Vec<String>,

    #[arg(help = "Do not print newline", short = 'n', long)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    let ending = if args.omit_newline { "" } else { "\n" };

    print!("{}{}", args.text.join(" "), ending)
}
