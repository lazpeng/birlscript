mod birl;
mod parser;

extern crate getopts;

use getopts::Options;
use parser::PEU;

/// Version of the package
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Help/Program usage
pub const USAGE: &'static str = "Ta querendo ajuda, cumpade?\n\t-a ou --ajuda-o-maluco-ta-doente: \
                                 Imprime essa mensagem de ajuda\n\t-c ou --compila-essa-porra      \
                                 : Compile o programa ao invés de interpretar";

fn print_help() {
    println!("BIRLSCRIPT v{} - Rafael R Nakano, Matheus Borella - 2016\n{}",
             VERSION,
             USAGE);
}

/// Return the input files passed to the program
fn get_input_files(args: &[String]) -> Vec<String> {
    let mut buffer: Vec<String> = vec![];
    for arg in args {
        if arg.chars().collect::<Vec<char>>()[0] == '-' {
            // First character is a '-', so is a flag or option
            continue;
        }
        buffer.push(arg.clone());
    }
    buffer
}

fn main() {
    use std::env;
    let arguments = env::args().collect::<Vec<String>>();
    let files = if arguments.len() < 2 {
        // No argument passed.
        vec![] // No files
    } else {
        let mut opts = Options::new();
        opts.optflag("a", "ajuda-o-maluco-ta-doente", "Imprime mensagem de ajuda");
        opts.optflag("c",
                     "compila-essa-porra",
                     "Compile o programa ao invés de interpretar");
        let matches = match opts.parse(&arguments[1..]) {
            Ok(m) => m,
            Err(e) => panic!("Erro interpretando flags do terminal: {}", e),
        };
        if matches.opt_present("a") {
            print_help(); // User asked for help message
        }
        if matches.opt_present("c") {
            // FIXME: Compile flag triggered
        }
        get_input_files(&arguments[1..])
    };
    let mut PEU = PEU::new(); // Creates a new executable unit
    for file in &files {
        PEU.parse_file(file); // Parse each file
    }
    // FIXME: Execute or compile files parsed in PEU
}
