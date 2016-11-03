pub mod kw;
pub mod global;
pub mod function;
pub mod command;

use self::global::{Global, GlobalParser};
use self::function::{Function, parse_functions};

/// Uma linha que possui um conteudo e numero de identificacao
pub type Line = (String, usize);

pub fn build_line(content: String, number: usize) -> Line {
    let result: (String, usize) = (content, number);
    result
}

pub struct AST {
    decl_globals: Vec<Global>,
    decl_functions: Vec<Function>,
}

#[derive(Debug)]
enum LineType {
    GlobalDeclaration,
    FunctionDeclaration,
    FunctionEnd,
    Other,
}

impl AST {
    fn new() -> AST {
        AST {
            decl_globals: vec![],
            decl_functions: vec![],
        }
    }

    pub fn declared_globals(&self) -> &Vec<Global> {
        &self.decl_globals
    }

    pub fn declared_functions(&self) -> &Vec<Function> {
        &self.decl_functions
    }

    pub fn load_file(name: &str) -> AST {
        use std::{io, fs};
        use std::io::BufRead;
        let f = fs::File::open(name)
            .unwrap_or_else(|err| panic!("Erro abrindo o arquivo \"{}\". {}", name, err));
        let mut buffer: Vec<String> = vec![];
        let reader = io::BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap_or_else(|error| panic!("Erro lendo do arquivo \"{}\". {}", name, error));
            buffer.push(line.clone());
        }
        AST::load(buffer)
    }

    pub fn load_string(src: &str) -> AST {
        let v: Vec<String> = src.split('\n').map(|cnt| cnt.to_owned()).collect();
        AST::load(v)
    }

    fn line_type(line: &str) -> LineType {
        if line.starts_with(kw::GLOBAL_VAR) || line.starts_with(kw::GLOBAL_CONST) {
            LineType::GlobalDeclaration
        } else if line.starts_with(kw::FUNCTION_START) {
            LineType::FunctionDeclaration
        } else if line.starts_with(kw::FUNCTION_END) {
            LineType::FunctionEnd
        } else {
            LineType::Other
        }
    }

    fn remove_comments(input: &str) -> String {
        if !input.contains(kw::COMMENT_CHAR) {
            input.to_owned()
        } else if input.is_empty() {
            String::new()
        } else {
            // Continua a pegar os caracteres enquanto não encontrar o caractere de comentario
            let result: String = input.chars()
                .take_while(|elem| *elem != kw::COMMENT_CHAR)
                .collect();
            result
        }
    }

    pub fn load(src: Vec<String>) -> AST {
        use std::thread;
        if src.is_empty() {
            AST::new()
        } else {
            let mut line_index = 1usize;
            // Buffers a serem interpretados
            let mut globals: Vec<Line> = vec![]; // Linhas que declaram um global
                // Vec de linhas que declaram funções
            let mut global_function_identifier = String::from(kw::FUNCTION_START) + " ";
            global_function_identifier.push_str(kw::SECT_GLOBAL);
            let mut functions: Vec<Vec<Line>> = vec![vec![build_line(global_function_identifier,
                                                                     0)]];
            let mut in_block = false; // Se o parser atualmente está em um bloco
            for line in src {
                line_index += 1;
                let usable = AST::remove_comments(line.trim());
                if usable.is_empty() {
                    continue;
                }
                match AST::line_type(&usable) {
                    LineType::FunctionDeclaration => {
                        if in_block {
                            panic!("Erro: Declaração de função dentro de outra função na linha {}",
                                   line_index);
                        }
                        functions.push(vec![build_line(line, line_index)]);
                        in_block = true;
                    }
                    LineType::FunctionEnd => {
                        if !in_block {
                            panic!("Erro no parsing, linha {}. Encerramento de função fora de \
                                    uma função.",
                                   line_index);
                        }
                        in_block = false;
                    }
                    LineType::GlobalDeclaration => globals.push(build_line(line, line_index)),
                    LineType::Other => {
                        // Se estiver num bloco (isto é, numa função definida pelo usuario, coloque no ultimo bloco)
                        // Senão, coloque no primeiro (a função global)
                        if in_block {
                            let index = functions.len() - 1;
                            functions[index].push(build_line(line, line_index));
                        } else {
                            functions[0].push(build_line(line, line_index));
                        }
                    }
                }
            }
            // Coloca um final na função global
            functions[0].push(build_line(String::from(kw::FUNCTION_END), 0));
            // Cria duas threads pra parsearem os valores e retorna a AST
            let parsed_globals =
                thread::spawn(move || {
                        let mut parser = GlobalParser::new();
                        parser.parse_globals(&globals)
                    })
                    .join()
                    .unwrap_or_else(|err| {
                        panic!("Erro na execução do parsing dos globais: {:?}", err)
                    });
            let parsed_functions = thread::spawn(move || parse_functions(functions))
                .join()
                .unwrap_or_else(|err| {
                    panic!("Erro na execução do parsing das funções: {:?}", err)
                });
            AST {
                decl_globals: parsed_globals,
                decl_functions: parsed_functions,
            }
        }
    }
}