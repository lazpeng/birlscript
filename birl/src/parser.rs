//! The parser for BirlScript
//!
#[cfg(target_pointer_width = "64")]
pub type IntegerType = i64;

#[cfg(target_pointer_width = "32")]
pub type IntegerType = i32;

const COMMENT_CHARACTER : char = '#';

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyPhrase {
    FunctionStart,
    FunctionEnd,
    PrintLn,
    Print,
    PrintDebug,
    Quit,
    Return,
    Declare,
    Set,
    Compare,
    EndSubScope,
    ExecuteIfEqual,
    ExecuteIfNotEqual,
    ExecuteIfEqualOrLess,
    ExecuteIfLess,
    ExecuteIfEqualOrGreater,
    ExecuteIfGreater,
    Call,
    GetStringInput,
    GetNumberInput,
    GetIntegerInput,
    IntoString,
    ConvertToInt,
    ConverToNum,
    TypeInt,
    TypeNum,
    TypeStr,
}

impl KeyPhrase {
    pub fn matches(src : &str) -> Option<KeyPhrase> {
        match src {
            "JAULA" => Some(KeyPhrase::FunctionStart),
            "SAINDO DA JAULA" => Some(KeyPhrase::FunctionEnd),
            "BIRL" => Some(KeyPhrase::Return),
            "NUM VAI DA NAO" |
            "NUM VAI DÁ NAO" |
            "NUM VAI DA NÃO" |
            "NUM VAI DÁ NÃO" => Some(KeyPhrase::Quit),
            "CE QUER VER" |
            "CÊ QUER VER" => Some(KeyPhrase::Print),
            "CE QUER VER ISSO" |
            "CÊ QUER VER ISSO" => Some(KeyPhrase::PrintLn),
            "VEM" => Some(KeyPhrase::Declare),
            "BORA" => Some(KeyPhrase::Set),
            "TRAPÉZIO DESCENDENTE" | "TRAPEZIO DESCENDENTE" => Some(KeyPhrase::TypeNum),
            "FIBRA" => Some(KeyPhrase::TypeStr),
            "BATATA DOCE" => Some(KeyPhrase::TypeInt),
            "E ELE QUE A GENTE QUER" |
            "É ELE QUE A GENTE QUER" => Some(KeyPhrase::Compare),
            "FIM" => Some(KeyPhrase::EndSubScope),
            "E HORA DO" | "É HORA DO" => Some(KeyPhrase::Call),
            "E ELE MEMO" | "É ELE MEMO" => Some(KeyPhrase::ExecuteIfEqual),
            "NUM E ELE" | "NUM É ELE" => Some(KeyPhrase::ExecuteIfNotEqual),
            "E MAIOR" | "É MAIOR" => Some(KeyPhrase::ExecuteIfGreater),
            "É MENOR" | "E MENOR" => Some(KeyPhrase::ExecuteIfLess),
            "MENOR OU E MEMO" | "MENOR OU É MEMO" => Some(KeyPhrase::ExecuteIfEqualOrLess),
            "MAIOR OU E MEMO" | "MAIOR OU É MEMO" => Some(KeyPhrase::ExecuteIfEqualOrGreater),
            "FALA AI" | "FALA AÍ" => Some(KeyPhrase::GetStringInput),
            "FALA UM NÚMERO" | "FALA UM NUMERO" => Some(KeyPhrase::GetNumberInput),
            "FALA AI UM INTEIRO" | "FALA AÍ UM INTEIRO" => Some(KeyPhrase::GetIntegerInput),
            "MUDA PRA TEXTO" => Some(KeyPhrase::IntoString),
            "MUDA PRA NUMERO" | "MUDA PRA NÚMERO" => Some(KeyPhrase::ConverToNum),
            "MUDA PRA INTEIRO" => Some(KeyPhrase::ConvertToInt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathOperator {
    Plus,
    Minus,
    Division,
    Multiplication,
    ParenthesisLeft,
    ParenthesisRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PonctuationKind {
    Colon,
    Comma,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Command(KeyPhrase),
    Symbol(String),
    Text(String),
    Number(f64),
    Integer(IntegerType),
    Operator(MathOperator),
    Ponctuation(PonctuationKind),
    Comment,
    NewLine,
    None
}

fn get_op(c : char) -> Option<MathOperator> {
    match c {
        '+' => Some(MathOperator::Plus),
        '-' => Some(MathOperator::Minus),
        '/' => Some(MathOperator::Division),
        '*' => Some(MathOperator::Multiplication),
        '(' => Some(MathOperator::ParenthesisLeft),
        ')' => Some(MathOperator::ParenthesisRight),
        _ => None,
    }
}

fn get_ponct(c : char) -> Option<PonctuationKind> {
    match c {
        ':' => Some(PonctuationKind::Colon),
        ',' => Some(PonctuationKind::Comma),
        _ => None,
    }
}

fn get_digit(c : char) -> Option<u8> {
    match c {
        '0' ... '9' => {
            let z = '0' as u8;
            let d = c as u8;

            Some(d - z)
        }
        _ => None
    }
}

fn number_token(input : &[char], offset : &mut usize, first : char) -> Result<Token, String> {

    let mut is_int = true;
    let mut int_val = 0 as IntegerType;
    let mut num_val = 0f64;
    let mut digits_after_dot = 0;

    if first == '.' {
        is_int = false;
        digits_after_dot = 1;
    } else {
        int_val = match get_digit(first) {
            Some(d) => d as IntegerType,
            None => return Err("Internal error : First char to number_token is not a digit or a dot".to_owned()),
        };
    }

    loop {
        if *offset >= input.len() {
            break;
        }

        let cur = input[*offset];

        if cur == COMMENT_CHARACTER {
            break;
        }

        if cur == '.' {
            if !is_int {
                return Err(String::from("Dois pontos aparecem no literal de número"));
            } else {
                is_int = false;
                num_val = int_val as f64;
                digits_after_dot = 1;
            }
        } else {
            match cur {
                '0'...'9' => {
                    let digit = get_digit(cur).unwrap();

                    if is_int {
                        int_val *= 10;
                        int_val += digit as IntegerType;
                    } else {
                        let diff = 0.1f64.powi(digits_after_dot);
                        num_val += diff * (digit as f64);
                        digits_after_dot += 1;
                    }
                }
                _ => break,
            }
        }

        *offset += 1;
    }

    if is_int {
        Ok(Token::Integer(int_val))
    } else {
        Ok(Token::Number(num_val))
    }
}

fn text_token(input : &[char], offset : &mut usize) -> Result<Token, String> {
    let mut content = String::new();

    let mut last_was_escape = false;

    loop {
        if *offset >= input.len() {
            break;
        }

        let cur = input[*offset];
        *offset += 1;

        if last_was_escape {
            match cur {
                '\\' |
                '\"' => content.push(cur),
                't' => content.push('\t'),
                'n' => content.push('\n'),
                'r' => content.push('\r'),
                _ => {} // warn?
            }

            last_was_escape = false;
        } else {
            match cur {
                '\"' => break,
                '\\' => last_was_escape = true,
                _ => content.push(cur),
            }
        }
    }

    Ok(Token::Text(content))
}

fn symbol_token(input : &[char], offset : &mut usize, first : char) -> Result<Token, String> {
    let mut result = String::new();

    result.push(first);

    let mut first_word = true;
    let mut first_char = false;

    loop {
        if *offset >= input.len() {
            break;
        }

        let cur = input[*offset];

        if cur == COMMENT_CHARACTER || cur == '\n' || cur == '\r' {
            break;
        }

        if cur == ' ' {
            if first_word {
                if let Some(kp) = KeyPhrase::matches(result.as_str()) {
                    return Ok(Token::Command(kp));
                }

                first_word = false;
            }

            if first_char {
                break;
            } else {
                first_char = true;
            }
        } else {
            if let Some(_) = get_op(cur) {
                break;
            }

            if let Some(_) = get_digit(cur) {
                if first_char {
                    break;
                }
            }

            if let Some(_) = get_ponct(cur) {
                break;
            }

            match cur {
                '.' => break,
                _ => {
                    if first_char {
                        result.push(' ');
                        first_char = false;
                    }

                    result.push(cur);
                },
            }
        }

        *offset += 1;
    }

    if let Some(kp) = KeyPhrase::matches(result.as_str()) {
        Ok(Token::Command(kp))
    } else {
        Ok(Token::Symbol(result))
    }
}

pub fn next_token(input : &[char], offset : &mut usize) -> Result<Token, String> {
    if *offset >= input.len() {
        return Ok(Token::None);
    }

    loop {
        if input[*offset] != ' ' && input[*offset] != '\t' {
            break;
        }

        *offset += 1;
    }

    let first_char = input[*offset];
    *offset += 1;

    if first_char == COMMENT_CHARACTER {
        return Ok(Token::Comment);
    }

    if first_char == '\n'  || first_char == '\r'{
        return Ok(Token::NewLine);
    }

    if let Some(op) = get_op(first_char) {
        return Ok(Token::Operator(op));
    }

    if let Some(p) = get_ponct(first_char) {
        return Ok(Token::Ponctuation(p));
    }

    if let Some(_) = get_digit(first_char) {
        return number_token(input, offset, first_char);
    }

    if first_char == '.' {
        return number_token(input, offset, first_char);
    }

    if first_char == '\"' {
        return text_token(input, offset);
    }

    symbol_token(input, offset, first_char)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeKind {
    Integer,
    Number,
    Text,
}

impl TypeKind {
    fn from_kp(kp : KeyPhrase) -> Option<TypeKind> {
        match kp {
            KeyPhrase::TypeInt => Some(TypeKind::Integer),
            KeyPhrase::TypeNum => Some(TypeKind::Number),
            KeyPhrase::TypeStr => Some(TypeKind::Text),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name : String,
    pub kind : TypeKind,
}

impl FunctionParameter {
    pub fn from(name : String, kind : TypeKind) -> FunctionParameter {
        FunctionParameter {
            name,
            kind
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name : String,
    pub arguments : Vec<FunctionParameter>,
}

impl FunctionDeclaration {
    pub fn from(name : String) -> FunctionDeclaration {
        FunctionDeclaration {
            name,
            arguments: vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MathValue {
    Integer(IntegerType),
    Number(f64),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    Value(MathValue),
    Symbol(String),
    Operator(MathOperator),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub nodes : Vec<ExpressionNode>,
    pub has_symbols : bool,
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            nodes: vec![],
            has_symbols: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandKind {
    Return,
    Quit,
    Print,
    PrintLn,
    PrintDebug,
    Declare,
    Set,
    Compare,
    EndSubScope,
    ExecuteIfEqual,
    ExecuteIfNotEqual,
    ExecuteIfEqualOrLess,
    ExecuteIfLess,
    ExecuteIfEqualOrGreater,
    ExecuteIfGreater,
    Call,
    GetStringInput,
    GetNumberInput,
    GetIntegerInput,
    ConvertToNum,
    ConvertToInt,
    IntoString,
}

impl CommandKind {
    fn from_kp(kp : KeyPhrase) -> Option<CommandKind> {
        match kp {
            KeyPhrase::Print => Some(CommandKind::Print),
            KeyPhrase::PrintLn => Some(CommandKind::PrintLn),
            KeyPhrase::PrintDebug => Some(CommandKind::PrintDebug),
            KeyPhrase::Return => Some(CommandKind::Return),
            KeyPhrase::Quit => Some(CommandKind::Quit),
            KeyPhrase::Declare => Some(CommandKind::Declare),
            KeyPhrase::Set => Some(CommandKind::Set),
            KeyPhrase::Compare => Some(CommandKind::Compare),
            KeyPhrase::EndSubScope => Some(CommandKind::EndSubScope),
            KeyPhrase::ExecuteIfEqual => Some(CommandKind::ExecuteIfEqual),
            KeyPhrase::ExecuteIfNotEqual => Some(CommandKind::ExecuteIfNotEqual),
            KeyPhrase::ExecuteIfEqualOrGreater => Some(CommandKind::ExecuteIfEqualOrGreater),
            KeyPhrase::ExecuteIfGreater => Some(CommandKind::ExecuteIfGreater),
            KeyPhrase::ExecuteIfEqualOrLess => Some(CommandKind::ExecuteIfEqualOrLess),
            KeyPhrase::ExecuteIfLess => Some(CommandKind::ExecuteIfLess),
            KeyPhrase::Call => Some(CommandKind::Call),
            KeyPhrase::GetStringInput => Some(CommandKind::GetStringInput),
            KeyPhrase::GetNumberInput => Some(CommandKind::GetNumberInput),
            KeyPhrase::IntoString => Some(CommandKind::IntoString),
            KeyPhrase::ConvertToInt => Some(CommandKind::ConvertToInt),
            KeyPhrase::ConverToNum => Some(CommandKind::ConvertToNum),
            KeyPhrase::GetIntegerInput => Some(CommandKind::GetIntegerInput),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum CommandArgumentKind {
    Name,
    Expression
}

struct CommandInfo {
    min_args : u32,
    max_args : i32,
    expected_args : Vec<CommandArgumentKind>
}

impl CommandInfo {

    fn from(min_args : u32, max_args : i32, expected_args : Vec<CommandArgumentKind>) -> CommandInfo {
        CommandInfo {
            min_args,
            max_args,
            expected_args
        }
    }

    fn from_kind(kind : CommandKind) -> CommandInfo {
        match kind {
            CommandKind::Quit => CommandInfo::from(0, 0, vec![]),
            CommandKind::Return => CommandInfo::from(0, 1,
                                                     vec![CommandArgumentKind::Expression]),
            CommandKind::Print => CommandInfo::from(1, -1,
                                                    vec![CommandArgumentKind::Expression]),
            CommandKind::PrintLn => CommandInfo::from(0, -1,
                                                      vec![CommandArgumentKind::Expression]),
            CommandKind::PrintDebug => CommandInfo::from(1, 1,
                                                         vec![CommandArgumentKind::Expression]),
            CommandKind::Declare => {
                CommandInfo::from(2, 2, vec![CommandArgumentKind::Name,
                                             CommandArgumentKind::Expression])
            }
            CommandKind::Set => {
                CommandInfo::from(2, 2, vec![CommandArgumentKind::Name,
                                             CommandArgumentKind::Expression])
            }
            CommandKind::Compare => {
                CommandInfo::from(2, 2, vec![CommandArgumentKind::Expression,
                                             CommandArgumentKind::Expression])
            }
            CommandKind::EndSubScope => {
                CommandInfo::from(0, 0, vec![])
            }
            CommandKind::Call => {
                CommandInfo::from(1, -1, vec![CommandArgumentKind::Name,
                                              CommandArgumentKind::Expression])
            }
            CommandKind::ExecuteIfEqual |
            CommandKind::ExecuteIfNotEqual |
            CommandKind::ExecuteIfLess |
            CommandKind::ExecuteIfGreater |
            CommandKind::ExecuteIfEqualOrLess |
            CommandKind::ExecuteIfEqualOrGreater => {
                CommandInfo::from(0, 0, vec![])
            }
            CommandKind::GetStringInput | CommandKind::GetNumberInput | CommandKind::IntoString |
            CommandKind::ConvertToNum | CommandKind::ConvertToInt | CommandKind::GetIntegerInput => {
                CommandInfo::from(1, 1, vec![CommandArgumentKind::Name])
            }
        }
    }
}

#[derive(Debug)]
pub enum CommandArgument {
    Name(String),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Command {
    pub kind : CommandKind,
    pub arguments : Vec<CommandArgument>,
}

#[derive(Debug)]
pub enum ParserResult {
    FunctionStart(FunctionDeclaration),
    FunctionEnd,
    Command(Command),
    Nothing,
}

fn parse_parameter(src : &[char], offset : &mut usize) -> Result<FunctionParameter, String> {
    let name = match next_token(src, offset) {
        Ok(Token::Symbol(s)) => s,
        Ok(t) => return Err(format!("Esperado um nome pro parâmetro, encontrado {:?}", t)),
        Err(e) => return Err(e)
    };

    match next_token(src, offset) {
        Ok(Token::Ponctuation(PonctuationKind::Colon)) => {} // OK,
        Ok(t) => return Err(format!("Esperado um : depois do nome, encontrado {:?}", t)),
        Err(e) => return Err(e)
    };

    let kind = match next_token(src, offset) {
        Ok(Token::Command(kp)) => {
            match TypeKind::from_kp(kp) {
                Some(t) => t,
                None => return Err(format!("Esperado um tipo pro parâmetro, mas {:?} não existe", kp)),
            }
        }
        Ok(t) => return Err(format!("Esperado um tipo pro parâmetro, encontrado {:?}", t)),
        Err(e) => return Err(e)
    };

    Ok(FunctionParameter::from(name, kind))
}

fn parse_function(src : &[char], offset : &mut usize) -> Result<ParserResult, String> {

    // Next token is the function name

    let name = match next_token(src, offset) {
        Ok(t) => {
            match t {
                Token::Symbol(name) => name,
                _ => return Err(format!("Esperado um nome pra função, encontrado um {:?}", t))
            }
        }
        Err(e) => return Err(e)
    };

    let mut func = FunctionDeclaration::from(name);

    match next_token(src, offset) {
        Ok(t) => {
           match t {
               Token::NewLine | Token::None => {}
               Token::Operator(MathOperator::ParenthesisLeft) => {
                   // Argument list

                   loop {
                       if *offset >= src.len() {
                           return Err("A lista de argumentos acaba incompleta".to_owned());
                       }

                       let param = match parse_parameter(src, offset) {
                           Ok(p) => p,
                           Err(e) => return Err(e)
                       };

                       func.arguments.push(param);

                       // Check next token

                       match next_token(src, offset) {
                           Ok(Token::Ponctuation(PonctuationKind::Comma)) => {} // Ok
                           Ok(Token::Operator(MathOperator::ParenthesisRight)) => break, // End
                           Ok(t) => return Err(format!("Esperado uma vírgula ou o fim da lista de parâmetros, encontrado {:?}", t)),
                           Err(e) => return Err(e),
                       };
                   }
               }
               _ => return Err(format!("Esperado o fim da declaração ou uma lista de parâmetros, encontrado {:?}", t)),
           }
        }
        Err(e) => return Err(e),
    }

    Ok(ParserResult::FunctionStart(func))
}

fn parse_sub_expression(src : &[char], offset : &mut usize, expr : &mut Expression, root : bool) -> Result<(), String> {

    let mut last_was_value;

    let mut dummy_offset = *offset;

    let first = match next_token(src, &mut dummy_offset) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    match first {
        Token::Comment | Token::None => return Ok(()),
        Token::Integer(i) => {
            last_was_value = true;

            expr.nodes.push(ExpressionNode::Value(MathValue::Integer(i)));
        }
        Token::Number(n) => {
            last_was_value = true;

            expr.nodes.push(ExpressionNode::Value(MathValue::Number(n)));
        }
        Token::Text(t) => {
            last_was_value = true;

            expr.nodes.push(ExpressionNode::Value(MathValue::Text(t)));
        }
        Token::NewLine => return Ok(()),
        Token::Symbol(s) => {
            last_was_value = true;

            if !expr.has_symbols {
                expr.has_symbols = true;
            }

            expr.nodes.push(ExpressionNode::Symbol(s));
        }
        Token::Operator(MathOperator::ParenthesisLeft) => {
            last_was_value = true;

            expr.nodes.push(ExpressionNode::Operator(MathOperator::ParenthesisLeft));

            match parse_sub_expression(src, &mut dummy_offset, expr, false) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        Token::Operator(MathOperator::ParenthesisRight) => {
            *offset = dummy_offset;

            expr.nodes.push(ExpressionNode::Operator(MathOperator::ParenthesisRight));

            return Ok(())
        },
        Token::Operator(o) => {
            match o {
                MathOperator::Plus | MathOperator::Minus => {
                    // Add a zero before this
                    expr.nodes.push(ExpressionNode::Value(MathValue::Integer(0)));
                    expr.nodes.push(ExpressionNode::Operator(o));
                },
                _ => return Err(format!("Scope ou expressão começa com o operator unário inválido {:?}", o)),
            }

            last_was_value = false;
        }
        Token::Ponctuation(p) => {
            match p {
                PonctuationKind::Comma if root => {
                    // Ok

                    return Ok(());
                }
                _ => return Err(format!("A expressão deveria começar com um valor ou operador unário, mas começa com {:?}", p)),
            }
        }
        _ => return Err(format!("Esperado um valor ou operador na expressão, encontrado {:?}", first)),
    }

    *offset = dummy_offset;

    loop {
        if *offset >= src.len() {
            break;
        }

        let current = match next_token(src, &mut dummy_offset) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        match current {
            Token::None | Token::Comment => return Ok(()),
            Token::Integer(i) => {
                if last_was_value {
                    return Err("Dois valores seguidos na expressão".to_owned());
                }

                last_was_value = true;

                expr.nodes.push(ExpressionNode::Value(MathValue::Integer(i)));
            }
            Token::Number(n) => {
                if last_was_value {
                    return Err("Dois valores seguidos na expressão".to_owned());
                }

                last_was_value = true;

                expr.nodes.push(ExpressionNode::Value(MathValue::Number(n)));
            }
            Token::Text(t) => {
                if last_was_value {
                    return Err("Dois valores seguidos na expressão".to_owned());
                }

                last_was_value = true;

                expr.nodes.push(ExpressionNode::Value(MathValue::Text(t)));
            }
            Token::Symbol(s) => {
                if last_was_value {
                    return Err("Dois valores seguidos na expressão".to_owned());
                }

                last_was_value = true;

                if !expr.has_symbols {
                    expr.has_symbols = true;
                }

                expr.nodes.push(ExpressionNode::Symbol(s));
            }
            Token::Operator(MathOperator::ParenthesisLeft) => {
                last_was_value = true;

                expr.nodes.push(ExpressionNode::Operator(MathOperator::ParenthesisLeft));

                match parse_sub_expression(src, &mut dummy_offset, expr, false) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                };
            }
            Token::Operator(MathOperator::ParenthesisRight) => {
                *offset = dummy_offset;

                expr.nodes.push(ExpressionNode::Operator(MathOperator::ParenthesisRight));

                break
            },
            Token::Operator(o) => {
                if !last_was_value {
                    return Err("Dois operadores seguidos na expressão".to_owned());
                }

                last_was_value = false;

                expr.nodes.push(ExpressionNode::Operator(o));
            }
            Token::Ponctuation(p) => {
                match p {
                    PonctuationKind::Comma if root => {
                        // Ok. Do not set offset to dummy_offset, since we want the lower calls and the parser to see the comma

                        break;
                    }
                    _ => return Err(format!("Erro: A expressão deveria começar com um valor ou operador unário, mas começa com {:?}", p)),
                }
            }
            Token::NewLine => break,
            _ => return Err(format!("Esperado um valor ou operador na expressão, encontrado {:?}", current)),
        }

        *offset = dummy_offset;
    }

    if !last_was_value {
        return Err("Expressão termina com um operador".to_owned());
    }

    Ok(())
}

fn parse_expression(src : &[char], offset : &mut usize) -> Result<Expression, String> {
    let mut expr = Expression::new();

    match parse_sub_expression(src, offset, &mut expr, true) {
        Ok(_) => {},
        Err(e) => return Err(e),
    };

    Ok(expr)
}

fn parse_command(src : &[char], offset : &mut usize, kp : KeyPhrase) -> Result<ParserResult, String> {
    let cmd_kind = match CommandKind::from_kp(kp) {
        Some(k) => k,
        // I don't think this will ever happen, so leave this awful message
        None => return Err("Invalid KeyPhrase to command".to_owned()),
    };

    let mut cmd = Command {
        kind : cmd_kind,
        arguments : vec![],
    };

    let info = CommandInfo::from_kind(cmd_kind);

    let mut has_arguments = if cmd_kind == CommandKind::PrintDebug {
        true
    } else {
        match next_token(src, offset) {
            Ok(t) => {
                match t {
                    Token::NewLine | Token::None => false,
                    Token::Ponctuation(PonctuationKind::Colon) => true,
                    _ => false,
                }
            }
            Err(e) => return Err(e),
        }
    };

    let mut dummy_offset = *offset;

    match next_token(src, &mut dummy_offset) {
        Ok(Token::NewLine) | Ok(Token::None) => has_arguments = false,
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    if has_arguments {
        let mut arg_index = 0usize;
        let mut arg_count = 0usize;

        loop {
            if *offset >= src.len() {
                break;
            }

            if arg_index >= info.expected_args.len() {
                if info.max_args < 0 {
                    arg_index = info.expected_args.len() - 1;
                } else {
                    return Err(format!("O comando espera, no máximo, apenas {} argumentos, mas mais que isso foi passado", info.max_args));
                }
            }

            let expected = info.expected_args[arg_index];

            match expected {
                CommandArgumentKind::Name => {
                    match next_token(src, offset) {
                        Ok(t) => {
                            match t {
                                Token::Symbol(s) => cmd.arguments.push(CommandArgument::Name(s)),
                                _ => return Err(format!("O argumento espera que o argumento #{} seja um nome, mas {:?} foi encontrado", arg_count, t)),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                CommandArgumentKind::Expression => {
                    let expr = match parse_expression(src, offset) {
                        Ok(e) => e,
                        Err(e) => return Err(e)
                    };

                    cmd.arguments.push(CommandArgument::Expression(expr));
                }
            }

            // Next token should be ',', or else the argument list is over

            let mut peek_offset = *offset;

            match next_token(src, &mut peek_offset) {
                Ok(t) => {
                    match t {
                        Token::None | Token::NewLine => break,
                        Token::Ponctuation(p) => {
                            match p {
                                PonctuationKind::Comma => {
                                    *offset = peek_offset;
                                } // OK
                                _ => return Err(format!("Esperado uma vírgula ou o fim dos argumentos, mas foi encontrado {:?}", p)),
                            }
                        }
                        _ => return Err(format!("Esperado uma vírgula ou o fim dos argumentos, mas foi encontrado {:?}", t)),
                    }
                }
                Err(e) => return Err(e),
            }

            arg_count += 1;
            arg_index += 1;
        }
    }

    if cmd.arguments.len() < info.min_args as usize {
        return Err(format!("O comando espera ao menos {} argumentos, mas {} foram passados", info.min_args, cmd.arguments.len()));
    }

    Ok(ParserResult::Command(cmd))
}

pub fn parse_line(src : &str) -> Result<ParserResult, String> {
    if src.trim().is_empty() {
        return Ok(ParserResult::Nothing);
    }

    let chars = src.chars().collect::<Vec<char>>();

    // try to infer what we're parsing from the first token

    let mut offset = 0usize;

    let first = match next_token(&chars, &mut offset) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    match first {
        Token::Comment => Ok(ParserResult::Nothing),
        Token::Command(kp) => {
            match kp {
                KeyPhrase::FunctionEnd => Ok(ParserResult::FunctionEnd),
                KeyPhrase::FunctionStart => parse_function(&chars, &mut offset),
                _ => parse_command(&chars, &mut offset, kp),
            }
        }
        Token::Text(_) | Token::Number(_) | Token::Integer(_) | Token::Symbol(_) => {
            offset = 0;
            parse_command(&chars, &mut offset, KeyPhrase::PrintDebug)
        }
        _ => Err("Linha começa com um token inválido".to_owned()),
    }
}

mod tests {
    #[test]
    fn functions() {
        use parser::*;

        {
            let src = "JAULA TESTANDO";

            let got_func = match parse_line(src) {
                Ok(ParserResult::FunctionStart(func)) => func,
                Ok(res) => panic!("Era esperado uma função, recebido {:?}", res),
                Err(e) => panic!("{}", e)
            };

            let expected = FunctionDeclaration::from("TESTANDO".to_owned());

            assert_eq!(got_func, expected);
        }

        {
            let src = "JAULA F(ARG1 : FIBRA, ARG2 : TRAPÉZIO DESCENDENTE)";

            let got_func = match parse_line(src) {
                Ok(ParserResult::FunctionStart(func)) => func,
                Ok(res) => panic!("Era esperado uma função, recebido {:?}", res),
                Err(e) => panic!("{}", e)
            };

            let mut expected = FunctionDeclaration::from("F".to_owned());
            expected.arguments.push(FunctionParameter::from("ARG1".to_owned(), TypeKind::Text));
            expected.arguments.push(FunctionParameter::from("ARG2".to_owned(), TypeKind::Number));

            assert_eq!(got_func, expected);
        }
    }

    #[test]
    fn numeric_tokens() {
        use parser::*;

        {
            let src = "1234";
            let chars = src.chars().collect::<Vec<char>>();
            let mut offset = 0usize;

            let tok = match next_token(&chars, &mut offset) {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };

            let expected = Token::Integer(1234);

            assert_eq!(tok, expected);
        }

        {
            let src = "1234.567";
            let chars = src.chars().collect::<Vec<char>>();
            let mut offset = 0usize;

            let tok = match next_token(&chars, &mut offset) {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };

            let expected = Token::Number(1234.567);

            assert_eq!(tok, expected);
        }
    }

    #[test]
    fn text_tokens() {
        use parser::*;

        let src = "\"test string\"";

        let chars = src.chars().collect::<Vec<char>>();

        let mut offset = 0usize;

        let tok = match next_token(&chars, &mut offset) {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };

        let expected = Token::Text("test string".to_owned());

        assert_eq!(tok, expected);
    }

    #[test]
    fn symbols_and_keyphrases() {
        use parser::*;

        {
            let src = "THIS IS A SYMBOL:+()1";
            let chars = src.chars().collect::<Vec<char>>();

            let mut offset = 0usize;

            let tok = match next_token(&chars, &mut offset) {
                Ok(t) => t,
                Err(e) => panic!("{}", e)
            };

            let expected = Token::Symbol("THIS IS A SYMBOL".to_owned());

            assert_eq!(tok, expected);
        }

        {
            let src = "NUM VAI DÁ NÃO";
            let chars = src.chars().collect::<Vec<char>>();

            let mut offset = 0usize;

            let tok = match next_token(&chars, &mut offset) {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };

            let expected = Token::Command(KeyPhrase::Quit);

            assert_eq!(tok, expected);
        }
    }
}
