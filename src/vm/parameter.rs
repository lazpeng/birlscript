
use super::variable;
use parser;

#[derive(Debug, Clone)]
/// Representa um parametro que pode ser passado como argumento para uma seção
pub struct Parameter {
    pub var: variable::Variable, // Contem o nome, valor e tipo
}

impl Parameter {
    /// Se todos os parametros coincidem em tipo e numero de argumentos
    pub fn matches(params: Vec<Parameter>, expected: Vec<parser::ExpectedParameter>) -> bool {
        if params.len() == expected.len() {
            let mut res = true;
            for i in 0..params.len() {
                let ref param = params[i];
                let ref expec = expected[i];
                if !(param.var.get_type() == expec.clone().tp) {
                    res = false;
                    break;
                }
            }
            res
        } else {
            false
        }
    }
}
