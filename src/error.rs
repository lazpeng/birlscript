//! Modulo responsável por reportar os erros
#![macro_use]

extern crate ansi_term;

#[macro_export]
macro_rules! abort {
    ($($tt:tt)*) => {{
        use std::process::*;
// No windows, a crate ansi_term não faz output colorido, então só printa a mensagem comum
        if cfg!(windows) {
            print!("Erro: ");
        } else {
            use ansi_term::*;
            print!("{}", Colour::Red.paint("Erro: "));
        }
        println!($($tt)*);
        exit(-1);
    }}
}

#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {{
        if cfg!(windows) {
            print!("Erro: ");
        } else {
            use ansi_term::*;
            print!("{}", Colour::Yellow.paint("Aviso: "));
        }
        println!($($tt)*);
    }}
}
