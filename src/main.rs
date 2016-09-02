mod error;
mod parser;
mod commands;
mod interpreter;
mod value;

extern crate ansi_term;

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
    println!("\t-t ou --tudo-cumpade                   : Imprime todos os comandos disponíveis");
    println!("\t-j ou --jaula [nome]                   : Diz ao interpretador pra usar outro \
              ponto de partida. Padrão: SHOW");
}

/// Versão numérica
pub static BIRLSCRIPT_VERSION: &'static str = "1.0.0";

/// Imprime a mensagem de versão
fn print_version() {
    println!("Versão dessa porra, cumpade:");
    println!("Interpretador BIRLSCRIPT v{}", BIRLSCRIPT_VERSION);
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
    /// Mostra todos os comandos disponiveis
    ShowCmds,
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
                "-" | "--" => warn!("Flag vazia passada."),
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
                            warn!("A flag \"-e ou --ele-que-a-gente-quer\" espera um \
                                      valor.");
                            break;
                        }
                    };
                    ret.push(Param::CommandHelp(cmd));
                }
                "-t" | "--tudo-cumpade" => ret.push(Param::ShowCmds),
                "-j" | "--jaula" => {
                    next_is_val = true;
                    let section = match params.next() {
                        Some(sect) => sect,
                        None => {
                            warn!("A flag \"-j ou --jaula\" espera um valor.");
                            break;
                        }
                    };
                    ret.push(Param::CustomInit(section));
                }
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

/// Imprime na tela todos os comandos disponíveis
fn show_cmds() {
    println!("Todos os comandos BIRL!");
    use parser::kw::*;
    let commands = vec![KW_MOVE,
                        KW_CLEAR,
                        KW_CMP,
                        KW_CMP_EQ,
                        KW_CMP_NEQ,
                        KW_CMP_LESS,
                        KW_CMP_LESSEQ,
                        KW_CMP_MORE,
                        KW_CMP_MOREEQ,
                        KW_DECL,
                        KW_DECLWV,
                        KW_JUMP,
                        KW_PRINT,
                        KW_PRINTLN,
                        KW_QUIT,
                        KW_INPUT,
                        KW_INPUT,
                        KW_INPUT_UP];
    for cmd in &commands {
        println!("{}", cmd);
    }
}

fn main() {
    let params = get_params();
    let mut files: Vec<String> = vec![];
    let mut env_default_sect = String::from(interpreter::BIRL_MAIN);
    let mut printed_something = false; // Se algo foi printado. Para que não jogue o erro quando pedir help, comandos ou version
    for p in params {
        match p {
            Param::PrintVersion => {
                printed_something = true;
                print_version()
            }
            Param::PrintHelp => {
                printed_something = true;
                print_help()
            }
            Param::CommandHelp(cmd) => {
                printed_something = true;
                command_help(&cmd)
            }
            Param::CustomInit(init) => env_default_sect = init,
            Param::InputFile(file) => files.push(file),
            Param::ShowCmds => {
                printed_something = true;
                show_cmds()
            }
        }
    }
    let mut environment = interpreter::Environment::new(env_default_sect);
    if files.len() > 0 {
        for file in files {
            environment.interpret(parser::parse(&file))
        }
        // Executa a jaula principal
        environment.start_program();
    } else {
        if !printed_something {
            abort!("Nenhum arquivo passado para a execução")
        }
    }
}
