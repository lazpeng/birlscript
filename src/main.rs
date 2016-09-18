mod error;
mod parser;
mod commands;
mod value;
mod vm;

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
    println!("\t-o ou --oloco-bixo                     : (DEBUG) Testa cada um dos exemplos pra \
              ter certeza que tá tudo funfando.");
}

/// Versão numérica
pub static BIRLSCRIPT_VERSION: &'static str = "1.1.5";

/// Imprime a mensagem de versão
fn print_version() {
    println!("Versão descendente:");
    println!("Interpretador BIRLSCRIPT v{}", BIRLSCRIPT_VERSION);
    println!("Copyleft(ɔ) 2016 Rafael R Nakano <mseqs@bsd.com.br> - Nenhum direito reservado");
}

/// Coleção de parametros passados ao interpretador
enum Param {
    /// Pedido para printar versão
    PrintVersion,
    /// Pedido para printar ajuda
    PrintHelp,
    /// Pedido para printar ajuda com um comando
    CommandHelp(String),
    /// Arquivo passado para interpretação
    InputFile(String),
    /// Mostra todos os comandos disponiveis
    ShowCmds,
    /// Testa todos os exemplos disponiveis
    Test,
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
                "-o" | "--oloco-bixo" => ret.push(Param::Test),
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
    let args = get_params();
    let mut printed_something = false;
    let mut files: Vec<String> = vec![];
    if args.len() > 0 {
        for arg in args {
            match arg {
                Param::PrintHelp => {
                    printed_something = true;
                    print_help()
                }
                Param::PrintVersion => {
                    printed_something = true;
                    print_version()
                }
                Param::CommandHelp(cmd) => {
                    printed_something = true;
                    command_help(&cmd)
                }
                Param::InputFile(file) => files.push(file),
                Param::ShowCmds => {
                    printed_something = true;
                    show_cmds()
                }
                Param::Test => {} // FIXME: Adicionar testes
            }
        }
    }
    if files.len() == 0 && !printed_something {
        abort!("Nenhum arquivo passado pra execução. Use -a ou --ajuda-o-maluco-ta-doente pra uma \
                lista de comandos pro interpretador.")
    }
    let units: Vec<parser::Unit> = files.into_iter().map(|f| parser::parse(&f)).collect();
    let mut vm = vm::VM::new(); // Cria a maquina virtual pra executar os comandos
    vm.load(units); // Carrega as unidades pra execução
    vm.start(); // Inicia a execução do programa
}
