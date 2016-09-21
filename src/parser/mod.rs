#![allow(dead_code)]

pub mod kw;

/// Abstract Syntax Tree, result of the parsing operation on a specific file
pub struct AST {

}

use std::rc::Rc;

/// Parsed Executable Unit. A collection of ASTs which can be directly executed by the vm
pub struct PEU {
    /// Collection of ASTs to be executed
    asts: Vec<AST>,
    /// A reference to the Unit which contains the main method to be executed
    main_unit: Option<Rc<AST>>,
}

impl PEU {
    /// Creates a new, empty, instance of a PEU
    pub fn new() -> PEU {
        PEU {
            asts: vec![],
            main_unit: None,
        }
    }

    /// Parses a new AST from a file and place it in asts
    pub fn parse_file(&mut self, file: &str) {
        // Retrive the lines from the file and call parse_lines
        use std::{fs, io};
        use std::io::BufRead;
        let handle = match fs::File::open(file) {
            Ok(h) => h,
            Err(err) => unimplemented!(), // FIXME: Erro
        };
        let mut buffer: Vec<String> = vec![]; // Where the lines are stored
        let reader = io::BufReader::new(handle); // Open a reader on the file
        for line in reader.lines() {
            let l = match line {
                Ok(ll) => ll,
                Err(err) => unimplemented!(), //FIXME: Erro
            };
            buffer.push(l);
        }
        self.parse_lines(buffer)
    }

    /// Parse a new AST from a string and place it in asts
    pub fn parse_str(&mut self, src: &str) {
        let buffer = src.split('\n').map(|line| String::from(line)).collect();
        self.parse_lines(buffer)
    }

    /// Parse a collection of lines
    fn parse_lines(&mut self, src: Vec<String>) {}
}
