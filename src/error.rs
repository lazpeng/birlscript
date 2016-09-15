//! Modulo responsável por reportar os erros
#![macro_use]

/// Aborta com uma mensagem de erro
#[macro_export]
macro_rules! abort {
    ($($tt:tt)*) => {{
        use std::process::*;
        print!("Erro: ");
        println!($($tt)*);
        exit(-1);
    }}
}

/// Emite um aviso a respeito de um problema, mas continua a execução do programa
#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {{
        print!("Aviso: ");
        println!($($tt)*);
    }}
}
