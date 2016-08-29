//! Modulo responsável pelos erros em BIRL

/// Da uma mensagem de erro e aborta
pub fn abort(error: &str) {
    use std::process;
    println!("Erro: {}", error);
    process::exit(-1);
}

/// Da uma mensagem de aviso
pub fn warn(warning: &str) {
    println!("Aviso: {}", warning);
}
