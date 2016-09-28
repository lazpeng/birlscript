#![allow(dead_code)]

mod global;
mod command;
mod function;
mod birlscript;

/// Abstract Syntax Tree, result of the parsing operation on a specific file
pub struct AST {
    globals: Vec<global::Global>,
    functions: Vec<function::Function>,
}

impl AST {
    /// Return the number of globals declared
    pub fn num_globals(&self) -> usize {
        self.globals.len()
    }

    pub fn from(globals: Vec<global::Global>, functions: Vec<function::Function>) -> AST {
        AST {
            globals: globals,
            functions: functions,
        }
    }

    /// Return a empty instance of an AST
    pub fn new() -> AST {
        AST {
            globals: vec![],
            functions: vec![],
        }
    }
}

use std::rc::Rc;

/// Parsed Executable Unit. A collection of ASTs which can be directly executed by the vm
pub struct PEU {
    /// Collection of ASTs to be executed
    pub asts: Vec<AST>,
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
            Err(err) => panic!("Erro abrindo arquivo \"{}\": {}", file, err), // FIXME: Erro
        };
        let mut buffer = String::new(); // The sum of all lines
        let reader = io::BufReader::new(handle); // Open a reader on the file
        for line in reader.lines() {
            let l = match line {
                Ok(ll) => ll,
                Err(err) => panic!("Erro lendo do arquivo \"{}\": {}", file, err), //FIXME: Erro
            };
            buffer.push_str(&l);
        }
        self.parse(&buffer)
    }

    /// Parse a new AST from a string and place it in asts
    pub fn parse(&mut self, src: &str) {
        // Parses the file, which returns an AST as result
        let result = match birlscript::parse_file(src) {
            Ok(res) => res,
            Err(e) => panic!("Erro no parsing do arquivo \"{}\": {:?}", src, e),
        };
        // Push the AST to the asts vector
        self.asts.push(result);
        match self.main_unit {
            None => {}  // No main unit set
            // TODO: Add code to detect if main unit is defined
            Some(_) => {}
        }
    }
}
