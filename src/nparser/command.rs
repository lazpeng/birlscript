use super::kw;
use super::Line;

#[derive(Debug, Clone)]
/// Representa um comando, que é executado dentro do contexto atual
/// Os valores passados aos comandos têm nomes fantasia alfabéticos para exemplificação
pub enum Command {
    /// Move (copia) o conteudo da variavel no endereco a pro b
    Move(String, String),
    /// Limpa o valor da variavel no endereco a
    Clear(String),
    /// Declara a variavel com nome a
    Decl(String),
    /// Declara a variavel com nome a e valor b
    DeclWV(String, String),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(String),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(String, String),
    /// Executa seção a caso ultima comparação seja igual
    CmpEq(Box<Command>),
    /// Executa seção caso a ultima comparação seja diferente
    CmpNEq(Box<Command>),
    /// Executa a seção caso a ultima comparação seja menor
    CmpLess(Box<Command>),
    /// Executa a seção caso a ultima comparação seja menor ou igual
    CmpLessEq(Box<Command>),
    /// Executa a seção caso a ultima comparação seja maior
    CmpMore(Box<Command>),
    /// Executa a seção caso a ultima comparação seja maior ou igual
    CmpMoreEq(Box<Command>),
    /// Printa uma série de valores com uma nova linha em seguida
    Println(Vec<String>),
    /// Printa uma série de valores
    Print(Vec<String>),
    /// Sai do programa
    Quit(String),
    /// Retorna da seção atual
    /// Valor, que pode existir ou não
    Return(Option<String>),
    /// Le a entrada padrão pra uma variavel
    Input(String),
    /// Le a entrada padrão e retorna um uppercase
    InputUpper(String),
}

impl Command {
    pub fn parse(line: Line) -> Command {
        let (cmd, indx) = line;
        // Estrutura de um comando:
        // COMANDO: var1, var2, ...
        let cmd_parts = split_command(&cmd);
        let cmd_parts = cmd_parts.iter().map(|part| part.trim()).collect::<Vec<&str>>();
        // argumentos
        let mut arguments: Vec<String> = Vec::new();
        // Tipo/nome do comando
        let cmd_type = if cmd_parts.len() > 1 {
            if num_args(cmd_parts[1]) == 0 {
                if cmd_parts[1].trim() != "" {
                    // Um argumento
                    arguments.push(cmd_parts[1].trim().to_owned());
                }
            } else {
                arguments = split_arguments(cmd_parts[1].trim().to_owned());
            }
            cmd_parts[0]
        } else {
            cmd.trim()
        };
        let cmd: Command = match cmd_type {
            kw::MOVE => {
                check_n_params(CommandType::Move, arguments.len(), indx);
                let (addr2, addr1) = (arguments.remove(1), arguments.remove(0));
                Command::Move(addr1, addr2)
            }
            kw::CLEAR => {
                check_n_params(CommandType::Clear, arguments.len(), indx);
                Command::Clear(arguments.remove(0))
            }
            kw::DECL => {
                check_n_params(CommandType::Decl, arguments.len(), indx);
                Command::Decl(arguments.remove(0))
            }
            kw::DECLWV => {
                check_n_params(CommandType::DeclWV, arguments.len(), indx);
                let (val, name) = (arguments.remove(1), arguments.remove(0));
                Command::DeclWV(name, val)
            }
            kw::JUMP => {
                // Jump requere uma gambiarra: As funções podem ter argumentos (',') adicionais, então use joint pra juntar os argumentos em 1 e retorne
                check_n_params(CommandType::Jump, arguments.len(), indx);
                Command::Jump(arguments.remove(0))
            }
            kw::CMP => {
                check_n_params(CommandType::Cmp, arguments.len(), indx);
                let (addr2, addr1) = (arguments.remove(1), arguments.remove(0));
                Command::Cmp(addr1, addr2)
            }
            kw::CMP_EQ => {
                check_n_params(CommandType::CmpEq, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpEq(Box::new(cmd))
            }
            kw::CMP_NEQ => {
                check_n_params(CommandType::CmpNEq, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpNEq(Box::new(cmd))
            }
            kw::CMP_LESS => {
                check_n_params(CommandType::CmpLess, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpLess(Box::new(cmd))
            }
            kw::CMP_LESSEQ => {
                check_n_params(CommandType::CmpLessEq, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpLessEq(Box::new(cmd))
            }
            kw::CMP_MORE => {
                check_n_params(CommandType::CmpMore, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpMore(Box::new(cmd))
            }
            kw::CMP_MOREEQ => {
                check_n_params(CommandType::CmpMoreEq, arguments.len(), indx);
                let cmd = Command::parse((arguments.remove(0), indx));
                Command::CmpMoreEq(Box::new(cmd))
            }
            kw::PRINTLN => {
                check_n_params(CommandType::Println, arguments.len(), indx);
                Command::Println(arguments.iter().map(|arg| arg.clone()).collect::<Vec<String>>())
            }
            kw::PRINT => {
                check_n_params(CommandType::Print, arguments.len(), indx);
                Command::Print(arguments.iter().map(|arg| arg.clone()).collect::<Vec<String>>())
            }
            kw::QUIT => {
                check_n_params(CommandType::Quit, arguments.len(), indx);
                let exitcode = if arguments.len() == 1 {
                    // Se for 1 argumento, retorne o valor, se não, deixe inalterado
                    arguments.remove(0)
                } else {
                    "0".to_owned()
                };
                Command::Quit(exitcode)
            }
            kw::RET => {
                check_n_params(CommandType::Return, arguments.len(), indx);
                let val = if arguments.len() == 1 {
                    // Se for 1 argumento, retorne o valor, se não, deixe inalterado
                    Some(arguments[0].clone())
                } else {
                    None
                };
                Command::Return(val)
            }
            kw::INPUT => {
                check_n_params(CommandType::Input, arguments.len(), indx);
                Command::Input(arguments.remove(0))
            }
            kw::INPUT_UP => {
                check_n_params(CommandType::InputUpper, arguments.len(), indx);
                Command::InputUpper(arguments.remove(0))
            }
            _ => {
                panic!("Erro no parsing, Linha {}. Erro: Comando \"{}\" não existe.",
                       indx,
                       cmd_type)
            }
        };
        cmd
    }
}

#[derive(Debug)]
/// Facil representação dos comandos sem os argumentos
pub enum CommandType {
    Move,
    Clear,
    Decl,
    DeclWV,
    Jump,
    Cmp,
    CmpEq,
    CmpNEq,
    CmpLess,
    CmpLessEq,
    CmpMore,
    CmpMoreEq,
    Println,
    Print,
    Quit,
    Return,
    Input,
    InputUpper,
}

/// Troca caracteres acentuados para suas versões sem acento
fn change_accents(src: &str) -> String {
    let mut nstr = String::new();
    for c in src.chars() {
        nstr.push(match c {
            'Á' | 'Ã' | 'À' => 'A',
            'É' => 'E',
            'Õ' | 'Ô' => 'O',
            'Í' => 'I',
            _ => c,
        });
    }
    nstr
}

/// Verifica se foi passada a quantidade correta de argumentos para um comando
fn check_n_params(command: CommandType, num_params: usize, line_number: usize) {
    // Pra cada comando, retorne um valor inteiro para o numero de parametros
    let (expected, id) = match command {
        CommandType::Cmp => (2, kw::CMP),
        CommandType::CmpEq => (1, kw::CMP_EQ),
        CommandType::CmpNEq => (1, kw::CMP_NEQ),
        CommandType::CmpLess => (1, kw::CMP_LESS),
        CommandType::CmpLessEq => (1, kw::CMP_LESSEQ),
        CommandType::CmpMore => (1, kw::CMP_MORE),
        CommandType::CmpMoreEq => (1, kw::CMP_MOREEQ),
        CommandType::Jump => (1, kw::JUMP),
        CommandType::DeclWV => (2, kw::DECLWV),
        CommandType::Decl => (1, kw::DECL),
        CommandType::Clear => (1, kw::CLEAR),
        CommandType::Move => (2, kw::MOVE),
        // print e println aceitam mais de um argumento, então faça uma checagem adicional
        CommandType::Println => {
            // No caso do println, ele pode ser usado sem um argumento, assim printando apenas uma nova linha
            (num_params, kw::PRINTLN)
        }
        CommandType::Print => {
            // Print não
            if num_params < 1 {
                (1, kw::PRINT)
            } else {
                (num_params, kw::PRINT)
            }
        }
        CommandType::Quit => {
            // Quit pode tomar um valor de retorno como valor de saida, mas é opcional
            if num_params == 1 {
                (1, kw::QUIT)
            } else {
                (0, kw::QUIT)
            }
        }
        CommandType::Return => {
            // Se for passado o retorno, retorne ele. Se não, deixe inalterado
            if num_params == 1 {
                (1, kw::RET)
            } else {
                (0, kw::RET)
            }
        }
        CommandType::Input => (1, kw::INPUT),
        CommandType::InputUpper => (1, kw::INPUT_UP),
    };
    if expected != num_params {
        panic!("Erro no parsing de um comando. Linha: {}. Erro: \"{}\" espera {} parametros, \
                porém {} foram passados.",
               line_number,
               id,
               expected,
               num_params)
    }
}

/// Divide os argumentos de um comando
fn split_arguments(args: String) -> Vec<String> {
    if args == "" {
        vec![]
    } else {
        let mut result: Vec<String> = vec![];
        let (mut in_str, mut in_char, mut last_escape, mut in_sym, mut in_par) =
            (false, false, false, false, false);
        let mut num_op_par = 0; // Numero de parenteses abertos
        let mut last_arg = String::new();
        for c in args.chars() {
            match c {
                '\"' if in_str => {
                    if last_escape {
                        last_escape = false;
                        last_arg.push_str("\\\"");
                    } else {
                        in_str = false;
                        last_arg.push('\"');
                    }
                }
                '\"' => {
                    in_str = true;
                    last_arg.push('\"');
                }
                '\'' => {
                    last_arg.push(c);
                    if !in_str {
                        in_char = !in_char;
                    }
                }
                '\\' => {
                    if last_escape {
                        last_arg.push('\\');
                        last_escape = false;
                    } else {
                        last_escape = true;
                    }
                }
                'a'...'z' | 'A'...'Z' | '_' if !in_str && !in_char => {
                    in_sym = true;
                    last_arg.push(c);
                }
                '(' if in_sym => {
                    // Parenteses
                    num_op_par += 1; // Abriu um parentese
                    in_par = true;
                    last_arg.push(c);
                }
                ')' if in_par => {
                    if num_op_par <= 0 {
                        panic!("Parentese de fechamento sem nenhum abrindo!")
                    }
                    num_op_par -= 1;
                    if num_op_par <= 0 {
                        in_par = false;
                    }
                    last_arg.push(c);
                }
                ',' if in_sym && !in_par => {
                    in_sym = false;
                    result.push(last_arg.clone());
                    last_arg.clear();
                }
                ',' if !in_str && !in_char && !in_sym && !in_par => {
                    result.push(last_arg.clone());
                    last_arg.clear();
                }
                ' ' if !in_str && !in_char && !in_sym => {}
                _ => last_arg.push(c),
            }
        }
        if last_arg != "" {
            result.push(last_arg.clone());
        }
        result.iter().map(|arg| arg.trim().to_string()).collect::<Vec<String>>()
    }
}

/// Divide um comando em nome e argumentos
fn split_command(cmd: &str) -> Vec<String> {
    let mut has_args = true; // Se o comando possui argumentos
    let index = match cmd.find(':') {
        Some(i) => i,
        None => {
            has_args = false;
            cmd.len()
        }
    };
    let cmd_name = &cmd[..index];
    let cmd_args: &str = if has_args { &cmd[index + 1..] } else { "" };
    vec![cmd_name.to_string(), cmd_args.to_string()]
}

fn num_args(src: &str) -> i32 {
    if src.len() <= 0 {
        0
    } else {
        let c = ',';
        let mut num = 0i32;
        let mut last_char = ' ';
        let mut is_string = false;
        for cur in src.chars() {
            if cur == '\"' || cur == '\'' {
                // String ou caractere, verifica o ultimo caractere
                if last_char == '\\' {
                    // caractere de escape, ignora
                } else {
                    // inicia ou finaliza string ou char
                    is_string = !is_string;
                }
            }
            if !is_string {
                if cur == c {
                    num += 1;
                }
            }
            last_char = cur;
        }
        num
    }
}