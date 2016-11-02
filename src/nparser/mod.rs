mod kw;
mod global;

use self::global::Global;

/// Uma linha que possui um conteudo e numero de identificacao
pub type Line<'a> = (&'a str, u64);

pub struct AST {
    decl_globals: Vec<Global>,
}
