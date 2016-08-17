//! Esse modulo serve somente para conter informações sobre os comandos

use parser::kw::*;

pub fn doc_move() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando copia o conteudo de B para a variavel A. B pode ser \
             um valor ou uma variavel (seu valor é usado)",
            KW_MOVE)
}

pub fn doc_clear() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel.\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando limpa o valor da variavel A",
            KW_CLEAR)
}

pub fn doc_xor() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando faz a operação binária XOR (eXclusive OR) na variavel \
             A com o valor B.",
            KW_XOR)
}
