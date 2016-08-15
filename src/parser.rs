//! Responsavel pelo parsing e de gerar a AST do programa BIRL

/// Representa as keywords da linguagem
mod kw {
    // Definições
    /// Usada para declaração de globais
    static KW_GLOBAL: &'static str = "SAI DE CASA";
    /// Usada para definição de seções
    static KW_SECTION: &'static str = "JAULA";
    
    // Comandos
    /// Copia o valor de uma variavel a outra
    static KW_MOVE: &'static str = "BORA";
    /// Limpa o valor de uma variavel
    static KW_CLEAR: &'static str = "NUM VAI DA NAO";
    /// Xor (operador binário)
    static KW_XOR: &'static str = "TRAPEZIO DESCENDENTE";
    /// And (operador binário)
    static KW_AND: &'static str = "FIBRA";
    /// Or (operador binário)
    static KW_OR: &'static str = "TRAPEZIO";
    /// Adição
    static KW_ADD: &'static str = "CONSTROI";
    /// Diminuição
    static KW_REM: &'static str = "NEGATIVA";
    /// Divisão
    static KW_DIV: &'static str = "AGUA COM MUSCULO";
    /// Multiplicação
    static KW_MUL: &'static str = "CONSTROI FIBRA";
    /// Declara uma variável
    static KW_DECL: &'static str = "VEM";
    /// Declara uma variável com um valor
    static KW_DECLWV: &'static str = "VEM PORRA";
    /// Realiza um "pulo" de uma seção para outra
    static KW_JUMP: &'static str = "HORA DO";
    /// Comparação
    static KW_CMP: &'static str = "E ELE QUE A GENTE QUER";
}

/// Representa um valor que pode ser atribuido a uma variavel
pub enum Value {
    /// Numero inteiro de 64 bits
    Integer(i64),
    /// Numero de ponto flutuante de 64 bits
    FloatP(f64),
    /// Caractere UTF-8
    Char(char),
    /// Texto em UTF-8, guarda apenas a referencia ao valor no heap
    Str(Box<String>),
}

/// Representa um comando, que é executado dentro do contexto atual
/// Os valores passados aos comandos têm nomes fantasia alfabéticos para exemplificação
pub enum Command {
    /// Move (copia) o conteudo da variavel no endereco a pro b
    Move(u64, u64),
    /// Limpa o valor da variavel no endereco a
    Clear(u64),
    /// Aplica xor na variavel no endereco a com o valor b
    Xor(u64, i64),
    /// Aplica and na variavel no endereco a com o valor b
    And(u64, i64),
    /// Aplica or  na variavel no endereco a com o valor b
    Or(u64, i64),
    /// Adiciona b ao valor da variavel no endereco a
    Add(u64, i64),
    /// Remove b do valor da variavel no endereco a
    Rem(u64, i64),
    /// Divide o valor da variavel no endereco a com o valor b
    Div(u64, i64),
    /// Multiplica o valor da variavel no endereco a com o valor b
    Mul(u64, i64),
    /// Declara a variavel com nome a
    Decl(String),
    /// Declara a variavel com nome a e valor b
    DeclWV(String, Value),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(String),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(u64, u64),
}

/// Faz parsing de um comando
fn parse_cmd(cmd: &str) -> Option<Command> {
    unimplemented!();
}

/// Representa uma unidade (arquivo compilado) contendo o conteudo a ser executado
pub struct Unit {
    /// Conjunto de seções para execução
    sects: Vec<Section>,
    /// Conjunto de globais
    consts: Vec<Global>,
}

/// Realiza a interpretação de um arquivo e retorna sua unidade compilada
pub fn parse(file: &str) -> Unit {
    unimplemented!();
}

/// Representa uma área chamável que pode ser executada
pub struct Section {
    /// Nome da seção
    name: String,
    /// Conjunto de linhas/comandos para execução
    lines: Vec<Command>,
}

/// Faz parsing de uma seção
fn parse_section(sect_str: &str) -> Section {
    unimplemented!();
}

/// Representa um valor global, constante
pub struct Global {
    /// Identificador do valor global
    identifier: String,
    /// Valor do global
    value: Value,
}

/// Faz parsing de um global
fn parse_global(glb: &str) -> Global {
    unimplemented!();
}