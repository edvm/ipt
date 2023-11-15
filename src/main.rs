use clap::Parser;
use std::path::PathBuf;
use text_extractor::extract_text;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file to extract text from 
    #[arg(short, long, value_name = "FILE")]
    extract: Option<PathBuf>,

}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.extract.as_deref() {
        match extract_text(path.to_str().unwrap()) {
            Ok(text) => {
                println!("{}", text);
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
