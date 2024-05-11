use clap::Parser;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The pattern to search for in your input.
    #[arg(short, long)]
    search: String,

    /// The replacement pattern.
    #[arg(short, long)]
    replace: String,

    /// The input content to search and replace. If not provided, input will be read from stdin.
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input = match args.input {
        Some(input) => input,
        None => {
            let mut input = String::new();
            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => input,
                Err(err) => {
                    eprintln!("Error reading from stdin: {}", err);
                    return;
                }
            }
        }
    };

    let output = dev_case::replace(&args.search, args.replace, input);
    print!("{}", output);
}
