use clap::Parser;

fn main() {
    let cli = warchest::Cli::parse();

    match cli.mode {
        warchest::Mode::Generate => {
            if let Err(e) = warchest::generate(&cli) {
                eprintln!("An error occured while generating: {e}");
            }
        }
        warchest::Mode::Fetch => {
            println!("we fetchin");
        }
    }
}
