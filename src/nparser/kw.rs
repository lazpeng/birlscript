// Definições
/// Usada para declaração de globais.birl constantes
pub const GLOBAL_CONST: &'static str = "SAI DE CASA";
/// Usada pra declaração de globais.birl variáveis
pub const GLOBAL_VAR: &'static str = "IBIRAPUERA";
/// Usada para definição de seções
pub const FUNCTION_START: &'static str = "JAULA";
/// Usada para finalizar a definição de seções
pub const FUNCTION_END: &'static str = "SAINDO DA JAULA";

/// Nome da variavel usada pra guardar o valor de retorno
pub const RETVAL_VAR: &'static str = "TREZE";

pub const SECT_GLOBAL: &'static str = "GLOBAL";
pub const SECT_DEFAULT: &'static str = "SHOW";

pub const COMMENT_CHAR: char = '#';

// Comandos
/// Copia o valor de uma variavel a outra
pub const MOVE: &'static str = "BORA";
/// Limpa o valor de uma variavel
pub const CLEAR: &'static str = "SAI";
/// Declara uma variável
pub const DECL: &'static str = "VEM";
/// Declara uma variável com um valor
pub const DECLWV: &'static str = "VEM, CUMPADE";
/// Realiza um "pulo" de uma seção para outra
pub const JUMP: &'static str = "E HORA DO";
/// Comparação
pub const CMP: &'static str = "E ELE QUE A GENTE QUER";
/// Comparação resultou em igual
pub const CMP_EQ: &'static str = "E ELE MEMO";
/// Comparação resultou em diferente
pub const CMP_NEQ: &'static str = "NUM E ELE";
/// Comparação resultou em menor
pub const CMP_LESS: &'static str = "MENOR, CUMPADE";
/// Comparação resultou em menor ou igual
pub const CMP_LESSEQ: &'static str = "MENOR OU E MEMO";
/// Comparação resultou em maior
pub const CMP_MORE: &'static str = "MAIOR, CUMPADE";
/// Comparação resultou em maior ou igual
pub const CMP_MOREEQ: &'static str = "MAIOR OU E MEMO";
/// Printa com nova linha
pub const PRINTLN: &'static str = "CE QUER VER ISSO";
/// Printa
pub const PRINT: &'static str = "CE QUER VER";
/// Sai do programa
pub const QUIT: &'static str = "NUM VAI DA NAO";
/// Retorna da função atual
pub const RET: &'static str = "BIRL";
/// Pega uma string da entrada padrão
pub const INPUT: &'static str = "BORA, CUMPADE";
/// Pega uma string da entrada padrão com letras maiusculas
pub const INPUT_UP: &'static str = "BORA, CUMPADE!!!";
