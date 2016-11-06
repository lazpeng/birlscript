
#[derive(Clone)]
/// Representa um sinal passado como resultado de um comando
pub enum Signal {
    /// Sinal pra saida do programa
    Quit(i32),
    /// Sinal pro retorno de uma seção
    Return,
}

use std::fmt;

impl fmt::Display for Signal {
    /// Torna possivel printar valores de Signal
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Signal::Quit(_) => write!(f, "Quit"),
            &Signal::Return => write!(f, "Return"),
        }
    }
}
