use eval::{Value, evaluate_raw, evaluate, ValueQuery};
use super::Line;
use std::collections::HashMap;
use super::kw;

#[derive(Clone, Debug)]
/// Variavel global, que pode ser constante ou não
pub struct Global {
    id: String,
    val: Value,
    is_const: bool,
}

impl Global {
    pub fn from(id: Option<String>, val: Option<Value>, is_const: Option<bool>) -> Global {
        Global {
            id: id.unwrap_or(String::new()),
            val: val.unwrap_or(Value::NullOrEmpty),
            is_const: is_const.unwrap_or(false),
        }
    }

    pub fn identifier(&self) -> &str {
        &self.id
    }

    pub fn value(&self) -> &Value {
        &self.val
    }

    /// Faz o parsing de um global. Separa a keyword do identificador e do valor, faz o parsing do valor e coloca o valor junto com o identificador no hash pra futuros globais
    fn parse_global<'a, QObj>(src: &str, query_obj: &'a QObj) -> Result<Global, &'static str>
        where QObj: ValueQuery + 'a
    {
        let start;
        let is_const;
        if src.starts_with(kw::GLOBAL_VAR) {
            // Global variavel
            start = kw::GLOBAL_VAR.len() + 1;
            is_const = false;
        } else if src.starts_with(kw::GLOBAL_CONST) {
            start = kw::GLOBAL_CONST.len() + 1;
            is_const = true;
        } else {
            return Err("A declaração da constante não começa com nenhuma das palavras chave");
        }
        let slice = &src[start..].trim();
        let space = match slice.find(' ') {
            Some(v) => v,
            None => return Err("Não há espaços após o identificador do global."),
        };
        let (identifier, value) = (slice[..space].to_owned(), &slice[space + 1..].trim());
        let evaluated = evaluate_raw(value, query_obj);
        let evaluated = evaluate(&evaluated, query_obj);
        Ok(Global::from(Some(identifier), Some(evaluated), Some(is_const)))
    }
}

/// Faz o parsing dos globals
pub struct GlobalParser {
    value_stack: HashMap<String, String>,
}

impl GlobalParser {
    pub fn new() -> GlobalParser {
        GlobalParser { value_stack: HashMap::new() }
    }

    pub fn parse_globals(&mut self, lines: &Vec<Line>) -> Vec<Global> {
        // Faz o parsing de multiplos globals, sendo que um pode referenciar o outro e seus valores são computados em tempo de processamento
        let mut global_vec: Vec<Global> = vec![];
        for line in lines {
            let &(ref line, ref number) = line;
            let result = match Global::parse_global(line, self) {
                Ok(res) => res,
                Err(e) => {
                    panic!("Erro na declaração de um global. Erro (linha: {}): {}",
                           *number,
                           e)
                }
            };
            global_vec.push(result);
        }
        global_vec
    }
}

impl ValueQuery for GlobalParser {
    fn query(&self, _id: &str) -> Option<Value> {
        // Pesquisa normal por Valor no GlobalParser não permitida
        unimplemented!()
    }

    fn query_raw(&self, id: &str) -> Option<String> {
        if self.value_stack.len() > 1 {
            let result = self.value_stack.get(id);
            match result {
                Some(x) => Some(x.clone()),
                None => None,
            }
        } else {
            None
        }
    }
}
