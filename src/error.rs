//! Modulo responsável por reportar os erros
#![macro_use]

/// Emite um aviso a respeito de um problema, mas continua a execução do programa
#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {{
        print!("Aviso: ");
        println!($($tt)*);
    }}
}
