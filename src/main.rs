use std::env;
use std::fs;
use std::process;

mod converter;

fn print_usage() {
    eprintln!("Usage: markdown-to-netscape <input.md> <output.html>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        process::exit(2);
    }
    let input = &args[1];
    let output = &args[2];

    if !input.ends_with(".md") {
        eprintln!("Error: input file must be a .md file");
        process::exit(3);
    }

    let src = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(4);
        }
    };

    match converter::convert_markdown_to_netscape(&src) {
        Ok(html) => {
            if let Err(e) = fs::write(output, html) {
                eprintln!("Error writing output file: {}", e);
                process::exit(5);
            }
            println!("Conversion completed: {}", output);
        }
        Err(err) => {
            eprintln!("Conversion error: {}", err);
            process::exit(6);
        }
    }
}
