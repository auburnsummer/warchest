use clap::Parser;
use warchest::{generate, Cli, Mode};

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Generate => {
            if let Err(e) = generate(&cli) {
                eprintln!("An error occured while generating: {e}");
            }
        }
        Mode::Fetch => {
            println!("we fetchin");
        }
    }
}