//! Modulo que representa as variaveis

use value::{Value, ValueType};
use parser;

pub enum Permission {
    /// Somente leitura (pode ser sobrescrita pela VM)
    ReadOnly,
    /// Leitura e escrita por todos globalmente
    ReadWriteAll,
    /// Leitura e escrita de um modulo especifico
    ReadWrite(String),
}

impl Permission {
    /// Verifica se um pedido de acesso é permitido de acordo com as permissões
    pub fn permitted(&self, acc: Access) -> bool {
        // Owner se refere a quem tem direito de acesso, no caso de escrita/leitura a um modulo especifico
        let global_perm = parser::kw::KW_SECT_GLOBAL.to_string();
        let (readonly, owner) = match self {
            &Permission::ReadOnly => (true, &global_perm),
            &Permission::ReadWriteAll => (false, &global_perm),
            &Permission::ReadWrite(ref who) => (false, who),
        };
        match acc.ac_type {
            AccessType::Read => {
                if *owner != global_perm {
                    // A variavel não está disponivel no contexto global, logo o dono deve ser o passado
                    if *owner == acc.from {
                        true // Acesso garantido
                    } else {
                        false // Não é o dono, acesso barrado
                    }
                } else {
                    true // Acesso garantido, variavel disponivel pra leitura globalmente
                }
            }
            AccessType::Write => {
                if readonly {
                    false // Variavel somente leitura,
                } else {
                    if *owner != global_perm {
                        if *owner == acc.from {
                            true // Dono
                        } else {
                            false // Não
                        }
                    } else {
                        // Variavel disponivel pra leitura e escrita globalmente
                        true
                    }
                }
            }
        }
    }
}

/// Tipo de acesso requisitado a uma variavel, checado de acordo com as permissões
pub enum AccessType {
    /// Leitura
    Read,
    /// Escrita
    Write,
}

/// Pedido de acesso a uma variavel, que contem o tipo de acesso e de onde vem
pub struct Access {
    /// Tipo de acesso
    ac_type: AccessType,
    /// De onde vem o pedido
    from: String,
}

impl Access {
    /// Um novo acesso com valores padrão
    pub fn new() -> Access {
        Access {
            ac_type: AccessType::Read,
            from: String::new(),
        }
    }

    /// Cria um novo acesso baseado nas informações passadas
    pub fn from(actype: AccessType, owner: &str) -> Access {
        Access {
            ac_type: actype,
            from: String::from(owner),
        }
    }
}

// Essa estrutura deve ser usada com cuidado. Como todas as seções compartilham
// uma mesma stack, todas as variaveis devem ser declaradas com o prefixo da seção.
// a variavel x da seção SHOW seria: __SHOW_x, pra evitar que ocorram problemas com
// permissões entre seções.
//

/// Estrutura que representa uma variavel
pub struct Variable {
    /// Identificador da Variavel
    pub id: String,
    /// Valor da variavel
    value: Value,
    /// Tipo do valor da variavel
    value_type: ValueType,
    /// Nivel de permissão de acesso à variavel
    access_perm: Permission,
}

impl Variable {
    /// Retorna uma nova variavel com os valores padrão
    pub fn new() -> Variable {
        Variable {
            id: String::new(),
            value: Value::Number(0.0),
            value_type: ValueType::Number,
            access_perm: Permission::ReadOnly,
        }
    }

    /// Cria uma nova variavel com o prefixo da seção
    pub fn from(name: &str, section: &str, value: Value, permission: Permission) -> Variable {
        let mut id = String::from("__");
        id.push_str(section);
        id.push('_');
        id.push_str(name);
        // No final fica __SEÇÃO_nome
        Variable {
            id: id,
            value_type: value.value_type(),
            value: value,
            access_perm: permission,
        }
    }

    /// Retorna o nome composto da variavel dependendo da seção
    pub fn make_name(var_name: &str, sect: &str) -> String {
        let mut res = String::from("__");
        res.push_str(sect);
        res.push('_');
        res.push_str(var_name);
        res
    }

    /// Retorna o nome da variavel sem o prefixo da seção
    pub fn real_id(&self) -> String {
        let last_underscore = match (&self.id[2..]).find('_') {
            Some(und) => und,
            None => {
                abort!("Erro interno, nome interno da variável incorreto. Nome: \"{}\".",
                       self.id)
            }
        };
        String::from(&self.id[last_underscore + 1..])
    }

    /// Retorna o tipo da variavel
    pub fn get_type(&self) -> ValueType {
        self.value_type.clone()
    }

    /// Tenta modificar a variavel
    pub fn modify(&mut self, new_value: Value, from: &str) {
        let access = Access::from(AccessType::Write, from);
        // Verifica se as permissões estão corretas
        if !self.access_perm.permitted(access) {
            abort!("Acesso de escrita não permitido em \"{}\", vindo de \"{}\"",
                   self.id,
                   from);
        }
        self.value_type = new_value.value_type();
        self.value = new_value;
    }

    /// Tenta pegar o valor da variavel
    pub fn retrieve_value(&self, from: &str) -> Value {
        let access = Access::from(AccessType::Read, from);
        if !self.access_perm.permitted(access) {
            abort!("Acesso de leitura não permitido em \"{}\", vindo de \"{}\"",
                   self.id,
                   from)
        }
        self.value.clone() // Retorna o valor da variavel
    }
}
