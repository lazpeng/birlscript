use eval;
use super::kw;
use super::{Line, build_line};
use super::command::Command;

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

/// Faz parsing da chamada de uma seção
pub fn parse_function_call_params(call: &str) -> (Vec<String>, String) {
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

/// Faz parsing da lista de argumentos que uma seção recebe
fn parse_function_parameters(decl_line: &str) -> Vec<ExpectedParameter> {
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

pub struct Function {
    identifier: String,
    inner_commands: Vec<Command>,
    expected_parameters: Vec<ExpectedParameter>,
}

impl Function {
    fn new() -> Function {
        Function {
            identifier: String::new(),
            inner_commands: vec![],
            expected_parameters: vec![],
        }
    }

    fn parse_header(line: &str, line_number: usize) -> (String, Vec<ExpectedParameter>) {
        if line.len() < kw::FUNCTION_START.len() {
            panic!("Buffer recebido não possui tamanho suficiente pra ter a keyword de \
                    declaração de função. Linha: {}",
                   line_number);
        }
        if !line.contains(' ') {
            panic!("Declaração de função na linha {} não possui um identificador.",
                   line_number);
        }
        let expected: Vec<ExpectedParameter> = if line.contains('(') {
            let ref buffer = line[kw::FUNCTION_START.len() + 1..];
            let space: i64 = match buffer.find(' ') {
                Some(s) => s as i64,
                None => -1,
            };
            let ref params = &buffer[(space + 1) as usize..];
            parse_function_parameters(params)
        } else {
            vec![]
        };
        let space = match line.find(' ') {
            Some(s) => s,
            None => unreachable!(),
        };
        let identifier = line[..space as usize].to_owned();
        (identifier, expected)
    }

    fn from(buffer: Vec<Line>) -> Function {
        if buffer.len() < 2 {
            let (_, number) = buffer[0];
            panic!("Erro no parsing da função, começando na linha {}: Não possui linhas \
                    suficientes.",
                   number);
        }
        let (ref content, number) = buffer[0];
        let (identifier, parameters) = Function::parse_header(content, number);
        // Joga fora a primeira e ultima linha, onde fica o fim da declaração da função
        let ref commands_buffer = buffer[1..buffer.len() - 2];
        let mut commands: Vec<Command> = vec![];
        for command_buffer in commands_buffer {
            let (line_content, number) = command_buffer.clone();
            commands.push(Command::parse(build_line(line_content, number)));
        }
        Function {
            identifier: identifier,
            inner_commands: vec![],
            expected_parameters: parameters,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.inner_commands
    }

    pub fn get_parameters(&self) -> &Vec<ExpectedParameter> {
        &self.expected_parameters
    }
}

pub fn parse_functions(buffers: Vec<Vec<Line>>) -> Vec<Function> {
    if buffers.is_empty() {
        vec![]
    } else {
        buffers.into_iter().map(|buffer| Function::from(buffer)).collect()
    }
}