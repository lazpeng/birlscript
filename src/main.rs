mod old; // The old backend

use std::env::args;

use old::interpreter::Interpreter;

fn print_help() {
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
    println!("\t--frango\t\t\t\t: (DEBUG) Usa o backend velho e zoado (default por enquanto)");
    println!("\t--monstro\t\t\t\t: (DEBUG) Usa o novo e não tão zoado backend (ainda não funciona)");
}

pub static BIRLSCRIPT_VERSION: &'static str = "2.0.0-alpha";

fn print_version() {
    println!("Interpretador e Compilador BIRLSCRIPT v{}", BIRLSCRIPT_VERSION);
    println!("© 2016, 2017 Rafael Rodrigues Nakano <lazpeng@gmail.com>");
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
    /// Uses the old backend (default for now)
    OldBackEnd(bool),
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
                "-s" | "--string" => {
                    // The next argument is expected to be a string containing source code
                    if let Some(code) = arguments.next() {
                        result.push(Param::StringSource(code));
                    } else {
                        println!("Erro: O argumento {} precisa de um conteúdo logo em seguida, bixo.", arg);
                    }
                }
                "--frango" => result.push(Param::OldBackEnd(true)),
                "--monstro" => result.push(Param::OldBackEnd(false)),
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
fn test_programs(_ : bool) {
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
    let mut old_backend = true;
    let mut test = false; // Should execute the test (so the backend and other properties can be set first)

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
                Param::Test => test = true,
                Param::StringSource(source) => string_sources.push(source),
                Param::OldBackEnd(w) => old_backend = w,
            }
        }
    }

    if test {
        test_programs(old_backend);
        did_something = true;
    }

    if files.is_empty() && !did_something && string_sources.is_empty() {
        panic!("Nenhum arquivo passado pra execução. Use -a ou --ajuda-o-maluco-ta-doente pra uma \
                lista de comandos pro interpretador.")
    }
    if old_backend {
        let mut interpreter = Interpreter::new();
        interpreter.load_files(files);
        interpreter.load_sources(string_sources);
        interpreter.start();
    } else {
        println!("O novo backend ainda não tá pronto, cumpade.");
    }
}
