//! Modulo responsável por reportar os erros
#![macro_use]

extern crate ansi_term;

#[macro_export]
macro_rules! abort {
    ($($tt:tt)*) => {{
        use std::process::*;
        use ansi_term::*;
        print!("{}", Colour::Red.paint("Erro: "));
        println!($($tt)*);
        exit(-1);
    }}
}

#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {{
        use ansi_term::*;
        print!("{}", Colour::Yellow.paint("Aviso: "));
        println!($($tt)*);
    }}
}
