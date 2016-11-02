//! Responsavel pelo parsing e de gerar a AST do programa BIRL

/// Representa as keywords da linguagem
pub mod kw {
    // Definições
    /// Usada para declaração de globais.birl constantes
    pub const KW_GLOBAL: &'static str = "SAI DE CASA";
    /// Usada pra declaração de globais.birl variáveis
    pub const KW_VAR_GLOBAL: &'static str = "IBIRAPUERA";
    /// Usada para definição de seções
    pub const KW_SECTION: &'static str = "JAULA";
    /// Usado pra definir seções parciais
    pub const KW_PART_SECTION: &'static str = "JAULINHA";
    /// Usada para finalizar a definição de seções
    pub const KW_SECTEND: &'static str = "SAINDO DA JAULA";

    /// Nome da variavel usada pra guardar o valor de retorno
    pub const KW_RETVAL_VAR: &'static str = "TREZE";

    pub const KW_SECT_GLOBAL: &'static str = "GLOBAL";
    pub const KW_SECT_DEFAULT: &'static str = "SHOW";

    // Comandos
    /// Copia o valor de uma variavel a outra
    pub const KW_MOVE: &'static str = "BORA";
    /// Limpa o valor de uma variavel
    pub const KW_CLEAR: &'static str = "SAI";
    /// Declara uma variável
    pub const KW_DECL: &'static str = "VEM";
    /// Declara uma variável com um valor
    pub const KW_DECLWV: &'static str = "VEM, CUMPADE";
    /// Realiza um "pulo" de uma seção para outra
    pub const KW_JUMP: &'static str = "E HORA DO";
    /// Comparação
    pub const KW_CMP: &'static str = "E ELE QUE A GENTE QUER";
    /// Comparação resultou em igual
    pub const KW_CMP_EQ: &'static str = "E ELE MEMO";
    /// Comparação resultou em diferente
    pub const KW_CMP_NEQ: &'static str = "NUM E ELE";
    /// Comparação resultou em menor
    pub const KW_CMP_LESS: &'static str = "MENOR, CUMPADE";
    /// Comparação resultou em menor ou igual
    pub const KW_CMP_LESSEQ: &'static str = "MENOR OU E MEMO";
    /// Comparação resultou em maior
    pub const KW_CMP_MORE: &'static str = "MAIOR, CUMPADE";
    /// Comparação resultou em maior ou igual
    pub const KW_CMP_MOREEQ: &'static str = "MAIOR OU E MEMO";
    /// Printa com nova linha
    pub const KW_PRINTLN: &'static str = "CE QUER VER ISSO";
    /// Printa
    pub const KW_PRINT: &'static str = "CE QUER VER";
    /// Sai do programa
    pub const KW_QUIT: &'static str = "NUM VAI DA NAO";
    /// Retorna da função atual
    pub const KW_RET: &'static str = "BIRL";
    /// Pega uma string da entrada padrão
    pub const KW_INPUT: &'static str = "BORA, CUMPADE";
    /// Pega uma string da entrada padrão com letras maiusculas
    pub const KW_INPUT_UP: &'static str = "BORA, CUMPADE!!!";
}

use eval;

#[derive(Clone)]
#[allow(dead_code)]
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

/// Procura pelo caractere c em src e retorna quantas vezes ele foi encontrado
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
fn check_n_params(command: CommandType, num_params: usize) {
    // Pra cada comando, retorne um valor inteiro para o numero de parametros
    let (expected, id) = match command {
        CommandType::Cmp => (2, kw::KW_CMP),
        CommandType::CmpEq => (1, kw::KW_CMP_EQ),
        CommandType::CmpNEq => (1, kw::KW_CMP_NEQ),
        CommandType::CmpLess => (1, kw::KW_CMP_LESS),
        CommandType::CmpLessEq => (1, kw::KW_CMP_LESSEQ),
        CommandType::CmpMore => (1, kw::KW_CMP_MORE),
        CommandType::CmpMoreEq => (1, kw::KW_CMP_MOREEQ),
        CommandType::Jump => (1, kw::KW_JUMP),
        CommandType::DeclWV => (2, kw::KW_DECLWV),
        CommandType::Decl => (1, kw::KW_DECL),
        CommandType::Clear => (1, kw::KW_CLEAR),
        CommandType::Move => (2, kw::KW_MOVE),
        // print e println aceitam mais de um argumento, então faça uma checagem adicional
        CommandType::Println => {
            // No caso do println, ele pode ser usado sem um argumento, assim printando apenas uma nova linha
            (num_params, kw::KW_PRINTLN)
        }
        CommandType::Print => {
            // Print não
            if num_params < 1 {
                (1, kw::KW_PRINT)
            } else {
                (num_params, kw::KW_PRINT)
            }
        }
        CommandType::Quit => {
            // Quit pode tomar um valor de retorno como valor de saida, mas é opcional
            if num_params == 1 {
                (1, kw::KW_QUIT)
            } else {
                (0, kw::KW_QUIT)
            }
        }
        CommandType::Return => {
            // Se for passado o retorno, retorne ele. Se não, deixe inalterado
            if num_params == 1 {
                (1, kw::KW_RET)
            } else {
                (0, kw::KW_RET)
            }
        }
        CommandType::Input => (1, kw::KW_INPUT),
        CommandType::InputUpper => (1, kw::KW_INPUT_UP),
    };
    if expected != num_params {
        panic!("\"{}\" espera {} parametros, porém {} foram passados.",
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
fn split_command(cmd: String) -> Vec<String> {
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

/// Faz parsing de um comando
fn parse_cmd(cmd: &str) -> Command {
    // Estrutura de um comando:
    // COMANDO: var1, var2, ...
    let cmd_parts = split_command(cmd.to_string());
    let cmd_parts = cmd_parts.iter().map(|part| part.trim()).collect::<Vec<&str>>();
    // argumentos
    let mut arguments: Vec<String> = Vec::new();
    // Tipo/nome do comando
    let cmd_type = if cmd_parts.len() > 1 {
        if num_args(cmd_parts[1]) == 0 {
            if cmd_parts[1].trim() != "" {
                // Um argumento
                arguments.push(cmd_parts[1].trim().to_string());
            }
        } else {
            arguments = split_arguments(cmd_parts[1].trim().to_string());
        }
        cmd_parts[0]
    } else {
        cmd.trim()
    };
    let cmd: Command = match cmd_type {
        kw::KW_MOVE => {
            check_n_params(CommandType::Move, arguments.len());
            let (addr1, addr2) = (arguments[0].clone(), arguments[1].clone());
            Command::Move(addr1, addr2)
        }
        kw::KW_CLEAR => {
            check_n_params(CommandType::Clear, arguments.len());
            Command::Clear(arguments[0].clone())
        }
        kw::KW_DECL => {
            check_n_params(CommandType::Decl, arguments.len());
            Command::Decl(arguments[0].clone())
        }
        kw::KW_DECLWV => {
            check_n_params(CommandType::DeclWV, arguments.len());
            let (name, val) = (arguments[0].clone(), arguments[1].clone());
            Command::DeclWV(name, val)
        }
        kw::KW_JUMP => {
            // Jump requere uma gambiarra: As funções podem ter argumentos (',') adicionais, então use joint pra juntar os argumentos em 1 e retorne
            check_n_params(CommandType::Jump, arguments.len());
            Command::Jump(arguments[0].clone())
        }
        kw::KW_CMP => {
            check_n_params(CommandType::Cmp, arguments.len());
            let (addr1, addr2) = (arguments[0].clone(), arguments[1].clone());
            Command::Cmp(addr1, addr2)
        }
        kw::KW_CMP_EQ => {
            check_n_params(CommandType::CmpEq, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpEq(Box::new(cmd))
        }
        kw::KW_CMP_NEQ => {
            check_n_params(CommandType::CmpNEq, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpNEq(Box::new(cmd))
        }
        kw::KW_CMP_LESS => {
            check_n_params(CommandType::CmpLess, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpLess(Box::new(cmd))
        }
        kw::KW_CMP_LESSEQ => {
            check_n_params(CommandType::CmpLessEq, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpLessEq(Box::new(cmd))
        }
        kw::KW_CMP_MORE => {
            check_n_params(CommandType::CmpMore, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpMore(Box::new(cmd))
        }
        kw::KW_CMP_MOREEQ => {
            check_n_params(CommandType::CmpMoreEq, arguments.len());
            let cmd = parse_cmd(&arguments[0]);
            Command::CmpMoreEq(Box::new(cmd))
        }
        kw::KW_PRINTLN => {
            check_n_params(CommandType::Println, arguments.len());
            Command::Println(arguments.iter().map(|arg| arg.clone()).collect::<Vec<String>>())
        }
        kw::KW_PRINT => {
            check_n_params(CommandType::Print, arguments.len());
            Command::Print(arguments.iter().map(|arg| arg.clone()).collect::<Vec<String>>())
        }
        kw::KW_QUIT => {
            check_n_params(CommandType::Quit, arguments.len());
            let exitcode = if arguments.len() == 1 {
                // Se for 1 argumento, retorne o valor, se não, deixe inalterado
                arguments[0].clone()
            } else {
                String::from("0")
            };
            Command::Quit(exitcode)
        }
        kw::KW_RET => {
            check_n_params(CommandType::Return, arguments.len());
            let val = if arguments.len() == 1 {
                // Se for 1 argumento, retorne o valor, se não, deixe inalterado
                Some(arguments[0].clone())
            } else {
                None
            };
            Command::Return(val)
        }
        kw::KW_INPUT => {
            check_n_params(CommandType::Input, arguments.len());
            Command::Input(arguments[0].clone())
        }
        kw::KW_INPUT_UP => {
            check_n_params(CommandType::InputUpper, arguments.len());
            Command::InputUpper(arguments[0].clone())
        }
        _ => panic!("Comando \"{}\" não existe.", cmd_type),
    };
    cmd
}

#[derive(Clone)]
/// Representa uma unidade (arquivo compilado) contendo o conteudo a ser executado
pub struct Unit {
    /// Conjunto de seções para execução
    pub sects: Vec<Section>,
    /// Conjunto de globais.birl, constantes ou variaveis
    pub globals: Vec<Global>,
}

/// Representa diferentes tipos de informação que uma linha carrega e o que representa
enum LineType {
    /// No caso de representar um comando
    Command,
    /// No caso de representar uma declaração de seção
    SectStart,
    /// No caso de representar o fim de uma seção
    SectEnd,
    /// Na declaração de um global
    GlobalDecl,
}

fn parse_line_type(line: &str) -> LineType {
    // line já foi usada trim()
    // testa se está finalizando uma seção
    if line == kw::KW_SECTEND {
        LineType::SectEnd
    } else {
        let mut ret = LineType::Command;
        // Se for a declaração de uma seção
        let fword = line.split(' ').collect::<Vec<&str>>()[0];
        if fword == kw::KW_SECTION || fword == kw::KW_PART_SECTION {
            ret = LineType::SectStart;
        }
        // Testa se é a declaração de um global
        let fword = line.split(':').collect::<Vec<&str>>()[0];
        if fword == kw::KW_GLOBAL || fword == kw::KW_VAR_GLOBAL {
            ret = LineType::GlobalDecl;
        }
        ret
    }
}

/// Realiza a interpretação de um arquivo e retorna sua unidade compilada
pub fn parse(file: &str) -> Unit {
    use std::fs;
    use std::io::{BufRead, BufReader};
    let f = match fs::File::open(file) {
        Ok(ff) => ff,
        Err(err) => {
            panic!("Não foi possivel abrir o arquivo \"{}\". Erro: {}",
                   file,
                   err)
        }
    };
    // Valor de retorno
    let mut final_unit = Unit {
        sects: vec![Section::new()],
        globals: vec![],
    };
    final_unit.sects[0].name = String::from(kw::KW_SECT_GLOBAL);
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    // Se está fazendo parsing de uma seção e o conteudo atual da seção
    let mut parsing_section = false;
    let mut cur_section: Vec<String> = vec![];
    loop {
        let line = match lines.next() {
            Some(l) => {
                match l {
                    Ok(ll) => String::from(ll.trim()),
                    Err(_) => String::new(),
                }
            }
            None => break,
        };
        if line == "" {
            continue;
        }
        // Retira os comentarios das linhas
        let line = if line.contains('#') || line.contains(';') {
            let mut tmp = String::new();
            for c in line.chars() {
                // Enquanto o caractere não for um comentário, continue
                if c != '#' && c != ';' {
                    tmp.push(c);
                } else {
                    break;
                }
            }
            tmp.trim().to_string()
        } else {
            line.trim().to_string()
        };
        if line == "" {
            // Depois de tirar os comentarios, a linha ficou vazia
            continue;
        }
        // Verifica a primeira palavra da linha
        match parse_line_type(&line) {
            // Se for declaração de um global, empurra o global pra unit
            LineType::GlobalDecl if !parsing_section => {
                final_unit.globals.push(parse_global(&line))
            }
            // Se for declaração de uma seção, começa o parsing da seção
            LineType::SectStart if !parsing_section => {
                cur_section.push(line.clone());
                parsing_section = true;
            }
            LineType::SectEnd if parsing_section => {
                cur_section.push(line.clone());
                if line == kw::KW_SECTEND {
                    // Encerra seção
                    parsing_section = false;
                    final_unit.sects.push(parse_section(cur_section.clone()));
                    cur_section.clear();
                }
            }
            // Quando estiver dentro de uma seção, empurre o comando pra seção
            LineType::Command if parsing_section => {
                // Mude os acentos para que aceite comandos com acento
                cur_section.push(change_accents(&line));
            }
            // Se não for nenhuma (os comandos só são interpretados dentro da seção)
            // A primeira seção sempre é a global, por isso puxe pra primeira seção
            LineType::Command => final_unit.sects[0].lines.push(parse_cmd(&change_accents(&line))),
            _ => {
                // Quando não for nenhuma das acima
                panic!("Erro de sintaxe! Linha atual não reconhecida no contexto: {}",
                       line)
            }
        }
    }
    final_unit
}

#[derive(Clone)]
/// Representa uma área chamável que pode ser executada
pub struct Section {
    /// Nome da seção
    pub name: String,
    /// Conjunto de linhas/comandos para execução
    pub lines: Vec<Command>,
    /// Conjunto de parametros a serem passados para a seção ser executada
    pub param_list: Vec<ExpectedParameter>,
    /// Se a seção é apenas parcial
    pub partial: bool,
}

impl Section {
    pub fn new() -> Section {
        Section {
            name: String::new(),
            lines: vec![],
            param_list: vec![],
            partial: false,
        }
    }
}

#[derive(Clone)]
pub struct ExpectedParameter {
    /// Identificador do parametro
    pub id: String,
    /// Tipo que o parametro espera
    pub tp: eval::ValueType,
}

/// Faz parsing de um parametro
fn parse_parameter(param: &str) -> ExpectedParameter {
    let div_token = match param.find(':') {
        Some(pos) => pos,
        None => panic!("Parametro deve ter tipo declarado depois do nome, separado por um ':'"),
    };
    let param_id = &param[..div_token];
    let param_tp = match eval::ValueType::try_parse(&param[div_token + 1..]) {
        Some(tp) => tp,
        None => {
            panic!("Tipo inválido para parâmetro: {}",
                   &param[div_token + 1..])
        }
    };
    ExpectedParameter {
        id: param_id.trim().to_string(),
        tp: param_tp,
    }
}

/// Faz parsing da lista de argumentos que uma seção recebe
fn parse_section_parameters(decl_line: &str) -> Vec<ExpectedParameter> {
    let decl_line = decl_line.trim();
    if !decl_line.contains('(') {
        vec![] // Nenhum argumento, retorna um array vazio
    } else {
        // Formato da declaração de uma seção com parametros:
        // JAULA seção (PARAMETRO1:TIPO, ...)
        let start_par = decl_line.find('(').unwrap(); // Ja verifiquei a existencia de um parentese
        if start_par >= decl_line.len() {
            panic!("Parametros declarados de forma incorreta. Parêntese em aberto");
        }
        let fin_par = decl_line.find(')').expect("Parêntese de fechamento não encontrado na declaração dos parametros da seção");
        if fin_par < start_par {
            panic!("Erro na sintaxe! Parêntese de fechamento veio antes do de abertura");
        }
        let parameters = decl_line[start_par + 1..fin_par].trim();
        if parameters == "" {
            vec![] // Retorna um array vazio, são só os parenteses nessa seção
        } else {
            if parameters.contains(',') {
                parameters.split(',').map(|param| parse_parameter(param.trim())).collect()
            } else {
                vec![parse_parameter(parameters)]
            }
        }
    }
}

/// Faz parsing de uma seção
fn parse_section(lines: Vec<String>) -> Section {
    // Separa a seção em linha
    if lines.len() < 2 {
        panic!("Erro fazendo parsing da seção. Número incorreto de linhas: {}.",
               lines.len())
    } else {
        // Checagens de declaração e finalização são feitas em parse
        // Declaração de uma seção:
        // PALAVRA_CHAVE nome
        if !lines[0].contains(' ') {
            panic!("Erro na declaração da seção! Falta nome depois da palavra chave")
        }
        let params = parse_section_parameters(&lines[0]);
        let first_space = lines[0].find(' ').unwrap(); // Primeira ocorrencia de espaco
        let name = if lines[0].contains('(') {
            // Se a declaração possui parametros, separa o nome dos parametros
            let starting_par = lines[0].find('(').unwrap();
            lines[0][first_space + 1..starting_par].trim().to_string()
        } else {
            lines[0][first_space + 1..].trim().to_string()
        };
        let sect_type = lines[0].split(' ').collect::<Vec<&str>>()[0];
        let is_part = sect_type == kw::KW_PART_SECTION;
        let mut sect = Section {
            name: name,
            lines: vec![],
            param_list: params,
            partial: is_part,
        };
        if lines.len() > 2 {
            // O -1 é pra não contar com a ultima linha, o SAINDO DA JAULA
            for line in lines[1..lines.len() - 1].iter() {
                if line == "" {
                    continue;
                }
                // Se a linha não tem nada de util até um comentario, pula ela
                if line.chars().collect::<Vec<char>>()[0] == '#' ||
                   line.chars().collect::<Vec<char>>()[0] == ';' {
                    continue;
                }
                sect.lines.push(parse_cmd(&line));
            }
        }
        sect
    }
}

/// Faz parsing da chamada de uma seção
pub fn parse_section_call_params(call: &str) -> (Vec<String>, String) {
    match call.find('(') {
        Some(op_par) => {
            if op_par >= call.len() - 1 {
                panic!("Parentese de abertura não possui fechamento");
            }
            let name = call[..op_par].to_string();
            let args = &call[op_par + 1..];
            let mut params: Vec<String> = vec![];
            let mut param_indx = 0; // Quantos parenteses foram abertos
            let mut last_arg = String::new();
            for c in args.chars() {
                if c == ')' {
                    if param_indx <= 0 {
                        break;
                    } else {
                        param_indx -= 1;
                    }
                } else if c == ')' {
                    param_indx += 1;
                } else if c == ',' {
                    if last_arg.trim() == "" {
                        panic!("Virgula após lista vazia de parametros passada.");
                    }
                    params.push(last_arg.clone());
                    last_arg.clear();
                } else {
                    last_arg.push(c);
                }
            }
            if param_indx != 0 {
                panic!("Parametros não fechados.");
            }
            if last_arg.trim() != "" {
                params.push(last_arg);
            }
            (params, name)
        }
        None => (vec![], call.to_string()), // Nenhum parametro passado
    }
}

#[derive(Clone)]
/// Representa um valor global, constante
pub struct Global {
    /// Identificador do valor global
    pub identifier: String,
    /// Valor do global
    pub value: String,
    /// Se o global é constante ou não
    pub is_const: bool,
}

/// Divide a declaração do global
fn split_global<'a>(glb: &'a str) -> Vec<&'a str> {
    let index = match glb.find(':') {
        Some(i) => i,
        None => panic!("Numero incorreto de ':' na declaração de um global."),
    };
    if index >= glb.len() - 1 {
        panic!("Faltam informações depois do primeiro ':'")
    }
    let nindex = match glb[index + 1..].find(':') {
        Some(i) => i,
        None => panic!("Numero incorreto de ':' na declaração de um global."),
    };
    if nindex >= glb.len() - 1 {
        panic!("Faltam informações após o segundo ':'")
    }
    vec![&glb[..index].trim(),
         &glb[index + 1..nindex + index + 1].trim(),
         &glb[nindex + index + 2..].trim()]
}

/// Faz parsing de um global
fn parse_global(glb: &str) -> Global {
    // Estrutura da declaração de um global: PALAVRA_CHAVE: nome: valor
    let words = split_global(glb.trim());
    if words.len() != 3 {
        panic!("Problema na declaração do global. Número incorreto de ':': {}",
               words.len())
    }
    let is_const = match words[0].trim() {
        kw::KW_GLOBAL => true,
        kw::KW_VAR_GLOBAL => false,
        _ => unreachable!(),
    };
    // Separa o nome e valor do global
    let (glb_name, glb_value) = (words[1].clone(), String::from(words[2]));
    Global {
        identifier: String::from(glb_name),
        value: glb_value,
        is_const: is_const,
    }
}
