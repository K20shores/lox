mod scanner;

use clap::Parser;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to compile
    #[arg(short, long)]
    file: Option<String>,
}

use scanner::Scanner;

fn run(source: String) {
    let s = Scanner::new(source);
    s.scan();
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut source = String::new();
        io::stdin()
            .read_line(&mut source)
            .expect("Failed to read line");
        source = source.trim_end_matches('\n').to_string();
        if source.is_empty() {
            return;
        }
        run(source);
    }
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        let contents = fs::read_to_string(file).expect("Should have been able to read the file");
        run(contents);
    } else {
        run_prompt();
    }
}
