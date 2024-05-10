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
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    match std::io::stdin().read_to_string(&mut input) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error reading from stdin: {}", err);
            return;
        }
    };

    let output = dev_case::replace(&args.search, args.replace, input);
    print!("{}", output);
}
