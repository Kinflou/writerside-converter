// Relative Modules
pub mod writerside;
pub mod convert;

#[cfg(test)]
pub mod tests;

// Standard Uses
use std::{path::PathBuf, process::ExitCode};

// External Uses
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "Input path for markdown docs")]
    input: PathBuf,

    #[clap(short, long, help = "Output path")]
    output: PathBuf,
}


fn main() -> std::process::ExitCode {
    println!("Markdown into Writerside topics converter\n");
    let args = Args::parse();

    let converted = convert::convert_docs(&args.input, &args.output);

    if cfg!(debug_assertions) {
        converted.unwrap()
    } else {
        match converted {
            Ok(_) => {},
            Err(e) => {
                println!("Couldn't convert docs: ");
                println!(" - {}", e);
    
                return ExitCode::FAILURE
            }
        }
    }


    println!("Markdown docs converted into Writerside project format.");
    return ExitCode::SUCCESS
}
