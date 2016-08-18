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

pub fn doc_and() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando faz a operação binária And na variavel \
             A com o valor B.",
            KW_AND)
}

pub fn doc_or() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando faz a operação binária Or na variavel \
             A com o valor B.",
            KW_OR)
}

pub fn doc_add() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
             B\nDescrição:\n\tEsse comando adiciona o valor B na variavel A. Funciona com números \
             e com strings.",
            KW_ADD)
}

pub fn doc_rem() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
         B\nDescrição:\n\tEsse comando remove o valor B da variavel A. Não funciona com strings",
            KW_REM)
}

pub fn doc_div() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
         B\nDescrição:\n\tEsse comando divide a variavel A com o valor B.",
            KW_DIV)
}

pub fn doc_mul() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel, Valor.\nExemplo:\n\t {0}: A, \
         B\nDescrição:\n\tEsse comando multiplica o valor de A pelo de B.",
            KW_MUL)
}

pub fn doc_neg() -> String {
    format!("Comando:\n\t \"{0}\".\nArgumentos:\n\t Variavel.\nExemplo:\n\t {0}: \
             A\nDescrição:\n\tEsse comando multiplica o valor em A por -1.",
            KW_NEG)
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
