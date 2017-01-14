mod nparser;
mod eval;
mod interpreter;

use interpreter::Interpreter;

extern crate meval;

/// Imprime mensagem de ajuda
fn print_help() {
    println!("Ta querendo ajuda, cumpade?");
    println!("O uso é o seguinte: birl [opções] [arquivo ou arquivos]");
    println!("Cê pode passar mais de um arquivo, só que apenas um pode ter a seção \"SHOW\", que \
              é");
    println!("o ponto de partida do teu programa.");
    println!("As opções são as seguintes:");
    println!("\t-a ou --ajuda-o-maluco-ta-doente       : Imprime essa mensagem de ajuda");
    println!("\t-v ou --vers[ã ou a]o-dessa-porra      : Imprime a versão do programa");
    println!("\t-o ou --oloco-bixo                     : (DEBUG) Testa cada um dos exemplos pra \
              ter certeza que tá tudo funfando.");
    println!("\t-s ou --string \"codigo\"             : Executa o codigo na string ao inves de \
              um arquivo.");
}

/// Versão numérica
pub static BIRLSCRIPT_VERSION: &'static str = "1.2.0";

/// Imprime a mensagem de versão
fn print_version() {
    println!("Versão descendente:");
    println!("Interpretador BIRLSCRIPT v{}", BIRLSCRIPT_VERSION);
    println!("Copyright(r) 2016 Rafael R Nakano <lazpeng@gmail.com> - Licença: MIT");
}

/// Coleção de parametros passados ao interpretador
enum Param {
    /// Pedido para printar versão
    PrintVersion,
    /// Pedido para printar ajuda
    PrintHelp,
    /// Arquivo passado para interpretação
    InputFile(String),
    /// Testa todos os exemplos disponiveis
    Test,
    /// Codigo via uma string
    StringSource(String),
}

/// Faz parsing dos comandos passados e retorna uma lista deles
fn get_params() -> Vec<Param> {
    use std::env;
    let mut ret: Vec<Param> = vec![];
    let mut params = env::args();
    // Se o proximo argumento é um valor que deve ser ignorado
    let mut next_is_val = false;
    if params.len() >= 2 {
        params.next(); // Se livra do primeiro argumento
        loop {
            let p = match params.next() {
                Some(v) => v,
                None => break,
            };
            if next_is_val {
                next_is_val = false;
                continue;
            }
            match p.as_str() {
                "-" | "--" => {}
                "-a" |
                "--ajuda-o-maluco-ta-doente" => ret.push(Param::PrintHelp),
                "-v" |
                "--versão-dessa-porra" |
                "--versao-dessa-porra" => ret.push(Param::PrintVersion),
                "-o" | "--oloco-bixo" => ret.push(Param::Test),
                "-s" | "--string" => {
                    let actual_source = match params.next() {
                        Some(x) => x,
                        None => {
                            panic!("Valor deve ser passado depois de -s, de preferencia entre \
                                    parenteses.")
                        }
                    };
                    ret.push(Param::StringSource(actual_source));
                }
                _ => ret.push(Param::InputFile(p)),
            }
        }
    }
    ret
}

/// Faz testes nos programas localizados em testes/
fn test_programs() {
    use std::{fs, thread, process};
    let files =
        fs::read_dir("testes").unwrap_or_else(|error| {
            panic!("Erro abrindo pasta de arquivos de testes: \"{:?}\"", error)
        });
    let mut suc_progs = 0u32;
    let mut unsuc_progs = 0u32; // Files not executed sucessfully
    let mut unsuc_files: Vec<String> = vec![];
    for file in files.into_iter() {
        let file = file.unwrap_or_else(|error| panic!("Erro abrindo arquivo: {:?}", error)).path();
        let file = match file.to_str() {
            Some(res) => res.to_string(),
            None => panic!("Erro recebendo caminho do arquivo"),
        };
        let filename = file.clone(); // Usado pelo unsuc_files quando dá panic. Não deve gastar muita memoria
        println!("\n\tProcessando: \"{}\"", filename);
        let interpreter_load_n_run = move || {
            let mut interp = Interpreter::new();
            interp.load_file(&file);
            interp.start();
        };
        let panicked = thread::spawn(interpreter_load_n_run).join().is_err();
        if panicked {
            unsuc_progs += 1;
            unsuc_files.push(filename);
        } else {
            suc_progs += 1;
        }
    }
    println!("\nResultado dos testes. Total: {}\n\n\tº {} testes concluidos com sucesso.\n\tº {} \
              com erros.\n\tº Testes com erros: {:?}",
             suc_progs + unsuc_progs,
             suc_progs,
             unsuc_progs,
             unsuc_files);
    // Sai do programa
    process::exit(0);
}

fn main() {
    let args = get_params();
    let mut did_something = false;
    let mut files: Vec<String> = vec![];
    let mut string_sources: Vec<String> = vec![];
    if args.len() > 0 {
        for arg in args {
            match arg {
                Param::PrintHelp => {
                    did_something = true;
                    print_help()
                }
                Param::PrintVersion => {
                    did_something = true;
                    print_version()
                }
                Param::InputFile(file) => files.push(file),
                Param::Test => test_programs(),
                Param::StringSource(source) => string_sources.push(source),
            }
        }
    }
    if files.is_empty() && !did_something && string_sources.is_empty() {
        panic!("Nenhum arquivo passado pra execução. Use -a ou --ajuda-o-maluco-ta-doente pra uma \
                lista de comandos pro interpretador.")
    }
    let mut interpreter = Interpreter::new();
    interpreter.load_files(files);
    interpreter.load_sources(string_sources);
    interpreter.start();
}
