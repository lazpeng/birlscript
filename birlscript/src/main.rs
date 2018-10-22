extern crate birl;

use std::env::args;
use birl::context::Context;

fn print_help() {
    Context::print_version();

    println!("Ta querendo ajuda, cumpade?");
    println!("O uso é o seguinte: birl [opções] [arquivo ou arquivos]");
    println!("Cê pode passar mais de um arquivo, só que apenas um pode ter a seção \"SHOW\", que \
              é o ponto de partida do teu programa.");
    println!("As opções são as seguintes:");
    println!("\t-a ou --ajuda-o-maluco-ta-doente\t: Imprime essa mensagem de ajuda");
    println!("\t-v ou --versao\t\t\t\t: Imprime a versão do programa");
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

fn main() {
    let args = get_params();
    let mut interactive = false;

    let mut ctx = Context::new();

    if args.len() > 0 {
        for arg in args {
            match arg {
                Param::PrintHelp => print_help(),
                Param::Interactive => interactive = true,
                Param::PrintVersion => Context::print_version(),
                Param::InputFile(file) => {
                    match ctx.add_file(file.as_str()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Ocorreu um erro ao adicionar o arquivo \"{}\" pro contexto : {}",
                                     file.as_str(), e);
                            // Exit? continue?
                        }
                    }
                }
                Param::StringSource(source) => {
                    match ctx.add_source_string(source) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Erro ao adicionar string de código ao contexto : {}", e);
                        }
                    }
                }
            }
        }
    } else {
        interactive = true;
    }

    if interactive {
        ctx.start_interactive();
    } else {
        match ctx.start_program() {
            Ok(_) => {}
            Err(e) => println!("Erro de execução : {}", e),
        }
    }
}
