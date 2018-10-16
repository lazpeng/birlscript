mod birl;

use std::env::args;
use birl::context::Context;

fn print_help() {
    Context::print_version();

    println!("Ta querendo ajuda, cumpade?");
    println!("O uso é o seguinte: birl [opções] [arquivo ou arquivos]");
    println!("Cê pode passar mais de um arquivo, só que apenas um pode ter a seção \"SHOW\", que \
              é");
    println!("o ponto de partida do teu programa.");
    println!("As opções são as seguintes:");
    println!("\t-a ou --ajuda-o-maluco-ta-doente\t: Imprime essa mensagem de ajuda");
    println!("\t-v ou --versao\t\t\t\t: Imprime a versão do programa");
    println!("\t-t ou --testa-ai-cumpade\t\t: (DEBUG) Testa cada um dos exemplos pra \
              ter certeza que tá tudo funfando.");
    println!("\t-s ou --string \"[codigo]\"\t\t: Executa o codigo na string ao inves de \
              um arquivo.");
    println!("\t-i ou --interativo\t\t\t\t: Inicia um console interativo pra rodar códigos");
}

/// Parameters passed through the command line
enum Param {
    PrintVersion,
    PrintHelp,
    /// Add a file to be processed
    InputFile(String),
    /// Do all tests
    Test,
    /// Processes code from a given string
    StringSource(String),
    /// Starts an interactive console for running code
    Interactive,
}

fn get_params() -> Vec<Param> {
    let mut arguments = args();
    let mut result: Vec<Param> = vec![];

    let _ = arguments.next().unwrap(); // Dispose of the first argument

    loop {
        if let Some(arg) = arguments.next() {
            match arg.as_str() {
                "-a" | "--ajuda-o-maluco-ta-doente" => result.push(Param::PrintHelp),
                "-v" | "--versao-cumpade" => result.push(Param::PrintVersion),
                "-t" | "--testa-ai-cumpade" => result.push(Param::Test),
                "-i" | "--interativo" => result.push(Param::Interactive),
                "-s" | "--string" => {
                    // The next argument is expected to be a string containing source code
                    if let Some(code) = arguments.next() {
                        result.push(Param::StringSource(code));
                    } else {
                        println!("Erro: O argumento {} precisa de um conteúdo logo em seguida, bixo.", arg);
                    }
                }
                // Push the file to the result stack
                _ => result.push(Param::InputFile(arg))
            }
        } else {
            break;
        }
    }

    result
}

// FIXME : Rewrite me later on
fn test_programs() {
}

fn main() {
    let args = get_params();
    let mut did_something = false;
    let mut files: Vec<String> = vec![];
    let mut string_sources: Vec<String> = vec![];
    let mut test = false; // Should execute the test (so the backend and other properties can be set first)
    let mut interactive = false;

    if args.len() > 0 {
        for arg in args {
            match arg {
                Param::PrintHelp => {
                    did_something = true;
                    print_help();
                }
                Param::Interactive => interactive = true,
                Param::PrintVersion => {
                    did_something = true;
                    Context::print_version();
                }
                Param::InputFile(file) => files.push(file),
                Param::Test => test = true,
                Param::StringSource(source) => string_sources.push(source),
            }
        }
    }

    if test {
        test_programs();
        did_something = true;
    }

    let mut ctx = Context::new();

    if (files.is_empty() && !did_something && string_sources.is_empty()) || interactive {
        ctx.start_interactive();
    } else {
        for s in string_sources {
            match ctx.add_source_string(s.as_str()) {
                Ok(_) => {},
                Err(e) => println!("Erro adicionando string de código : {}", e),
            }
        }

        for f in files {
            match ctx.add_file(f.as_str()) {
                Ok(_) => {},
                Err(e) => println!("Erro no arquivo {} : {}", f.as_str(), e),
            }
        }

        match ctx.start_program() {
            Ok(_) => {}
            Err(e) => println!("Erro em start_program : {}", e),
        }
    }
}
