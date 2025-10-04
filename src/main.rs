use std::fs;
use std::path::PathBuf;
use std::process;

mod converter;
mod models;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Convert Markdown link lists to Netscape bookmark HTML"
)]
struct Args {
    /// Input markdown file (links list)
    input: PathBuf,

    /// Output HTML file (Netscape bookmark format)
    output: PathBuf,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    if let Some(ext) = args.input.extension() {
        if ext != "md" {
            eprintln!("Error: input file must have .md extension");
            process::exit(3);
        }
    }

    let src = match fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(4);
        }
    };

    match converter::convert_markdown_to_netscape(&src) {
        Ok(html) => {
            if let Err(e) = fs::write(&args.output, html) {
                eprintln!("Error writing output file: {}", e);
                process::exit(5);
            }
            println!("Conversion completed: {}", args.output.display());
        }
        Err(err) => {
            eprintln!("Conversion error: {:?}", err);
            process::exit(6);
        }
    }
}
