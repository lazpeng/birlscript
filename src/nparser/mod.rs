mod kw;
mod global;

/// Uma linha que possui um conteudo e numero de identificacao
pub type Line<'a> = (&'a str, u64);
