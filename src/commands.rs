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

pub fn doc_decl() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel (Nome).\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando declara uma nova variável com nome de A",
            KW_DECL)
}

pub fn doc_declwv() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel (Nome), Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando declara uma nova variável com nome de A e valor de B",
            KW_DECLWV)
}

pub fn doc_jump() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Seção (nome).\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando transfere a execução para a seção A",
            KW_JUMP)
}

pub fn doc_cmp() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Valor, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando verifica semelhanças e diferenças entre A e B para uso \
             em condicionais",
            KW_CMP)
}

pub fn doc_println() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel.\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando imprime o valor de A com uma nova linha",
            KW_PRINTLN)
}

pub fn doc_print() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel.\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando imprime o valor de A sem uma nova linha",
            KW_PRINT)
}

pub fn doc_quit() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t <NENHUM>.\nExemplo:\n\t \
             {0}\nDescrição:\n\tEsse comando faz o programa fechar na sua execução",
            KW_QUIT)
}
