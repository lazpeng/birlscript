//! Modulo que representa as variaveis

use value::{Value, ValueType};

#[derive(Debug, Clone)]
/// Estrutura que representa uma variavel
pub struct Variable {
    /// Identificador da Variavel
    id: String,
    /// Valor da variavel
    value: Value,
    /// Tipo do valor da variavel
    value_type: ValueType,
}

impl Variable {
    /// Retorna uma nova variavel com os valores padrão
    pub fn new() -> Variable {
        Variable {
            id: String::new(),
            value: Value::Number(0.0),
            value_type: ValueType::Number,
        }
    }

    /// Cria uma nova variavel com o prefixo da seção
    pub fn from(name: &str, value: Value) -> Variable {
        Variable {
            id: name.to_string(),
            value_type: value.value_type(),
            value: value,
        }
    }

    pub fn get_id<'a>(&'a self) -> &'a str {
        &self.id
    }

    pub fn get_val<'a>(&'a self) -> &'a Value {
        &self.value
    }

    /// Retorna o tipo da variavel
    pub fn get_type(&self) -> ValueType {
        self.value_type.clone()
    }

    /// Tenta modificar a variavel
    pub fn modify(&mut self, new_value: Value) {
        self.value_type = new_value.value_type();
        self.value = new_value;
    }
}
