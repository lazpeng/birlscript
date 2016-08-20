mod parser;
mod commands;
mod interpreter;

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
    println!("\t-e ou --ele-que-a-gente-quer [comando] : Imprime uma mensagem de ajuda para o \
              comando");
    println!("\t-j ou --jaula [nome]                   : Diz ao interpretador pra usar outro \
              ponto de partida. Padrão: SHOW");
    println!("\t-q ou --quer-ver-tudo                  : Diz ao interpretador para usar modo \
              verbose.");
    // TODO: Comando para ajustar stack size
}

/// Versão numérica
pub static BIRLSCRIPT_VERSION: &'static str = "0.1.3";
/// Release, como alfa, beta, etc
pub static BIRLSCRIPT_RELEASE: &'static str = "ALFA";

/// Imprime a mensagem de versão
fn print_version() {
    println!("Versão dessa porra, cumpade:");
    println!("Interpretador BIRLSCRIPT v{} - {}",
             BIRLSCRIPT_VERSION,
             BIRLSCRIPT_RELEASE);
    println!("Copyleft(ɔ) 2016 Rafael R Nakano - Nenhum direito reservado");
}

/// Coleção de parametros passados ao interpretador
enum Param {
    /// Pedido para printar versão
    PrintVersion,
    /// Pedido para printar ajuda
    PrintHelp,
    /// Pedido para printar ajuda com um comando
    CommandHelp(String),
    /// Pedido para modificar o ponto de partida
    CustomInit(String),
    /// Arquivo passado para interpretação
    InputFile(String),
    /// Pede ao compilador a ser verbose
    Verbose,
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
                "-a" |
                "--ajuda-o-maluco-ta-doente" => ret.push(Param::PrintHelp),
                "-v" |
                "--versão-dessa-porra" |
                "--versao-dessa-porra" => ret.push(Param::PrintVersion),
                "-e" |
                "--ele-que-a-gente-quer" => {
                    next_is_val = true;
                    let cmd = match params.next() {
                        Some(name) => name,
                        None => {
                            println!("Erro: a flag \"-e ou --ele-que-a-gente-quer\" espera um \
                                      valor.");
                            break;
                        }
                    };
                    ret.push(Param::CommandHelp(cmd));
                }
                "-j" | "--jaula" => {
                    next_is_val = true;
                    let section = match params.next() {
                        Some(sect) => sect,
                        None => {
                            println!("Erro: a flag \"-j ou --jaula\" espera um valor.");
                            break;
                        }
                    };
                    ret.push(Param::CustomInit(section));
                }
                "-q" |
                "--quer-ver-tudo" => ret.push(Param::Verbose),
                _ => ret.push(Param::InputFile(p)),
            }
        }
    }
    ret
}

/// Printa ajuda para um comando
fn command_help(command: &str) {
    use parser::kw::*;
    use commands::*;
    let doc = match command {
        KW_MOVE => doc_move(),
        KW_CLEAR => doc_clear(),
        KW_XOR => doc_xor(),
        KW_AND => doc_and(),
        KW_OR => doc_or(),
        KW_ADD => doc_add(),
        KW_REM => doc_rem(),
        KW_DIV => doc_div(),
        KW_MUL => doc_mul(),
        KW_NEG => doc_neg(),
        KW_DECL => doc_decl(),
        KW_DECLWV => doc_declwv(),
        KW_JUMP => doc_jump(),
        KW_CMP => doc_cmp(),
        KW_PRINTLN => doc_println(),
        KW_PRINT => doc_print(),
        KW_QUIT => doc_quit(),
        _ => String::from("Comando não encontrado"),
    };
    println!("{}", doc);
}

fn main() {
    let params = get_params();
    let mut files: Vec<String> = vec![];
    let mut env_params = interpreter::EnvironmentOptions::new();
    for p in params {
        match p {
            Param::PrintVersion => print_version(),
            Param::PrintHelp => print_help(),
            Param::CommandHelp(cmd) => command_help(&cmd),
            Param::CustomInit(init) => env_params.set_default_section(init),
            Param::InputFile(file) => files.push(file),
            Param::Verbose => env_params.set_verbose(true),
        }
    }
    let mut environment = interpreter::Environment::new(env_params);
    if files.len() > 0 {
        for file in files {
            environment.interpret(parser::parse(&file))
        }
        // Executa a jaula principal
        environment.start_program();
    } else {
        println!("Nenhum arquivo passado pra execução!");
    }
}
