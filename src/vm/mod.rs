//! Novo modulo responsavel pela execução dos codigos birlscript
#![allow(dead_code)]

mod variable;
mod section;
mod parameter;
mod signal;
mod command;
mod comparision;

use parser;
use eval::{ValueQuery, Value, ValueType, evaluate};

/// Tipo de identificador usado na VM
pub type VMID = u16;

/// Máximo possivel que uma seção pode executar recursão em si mesma
pub const MAX_RECURSION: usize = 124;

/// Onde são executados os comandos
pub struct VM {
    /// Seções executaveis presentes na VM
    sections: Vec<section::Section>,
    /// Stack de variaveis presente na seção
    stack: Vec<section::Section>,
    /// Ultimo sinal jogado à VM
    last_signal: Option<signal::Signal>,
    /// Ultima comparação
    last_cmp: comparision::Comparision,
}

impl ValueQuery for VM {
    fn query(&self, id: &str) -> Option<Value> {
        self.retrieve_variable(id)
    }

    fn query_raw(&self, _id: &str) -> Option<String> {
        unimplemented!()
    }
}

impl VM {
    /// Procura na pilha se seções e retorna uma referência para a seção
    pub fn get_section<'a>(&'a mut self, name: &str) -> &'a mut section::Section {
        // self.sections sempre tem ao menos um elemento, por isso a checagem não é necessaria
        for sect in &mut self.sections {
            if sect.name == name {
                return sect;
            }
        }
        panic!("Seção \"{}\" não declarada.", name); // Se chegou até aqui, é porquê não foi encontrada
    }

    /// Declara a variavel na stack da VM
    pub fn declare_variable(&mut self, var: variable::Variable) {
        self.current_section_mut().decl_var(var);
    }

    /// Tenta modificar a variavel com o nome passado.
    pub fn modify_variable(&mut self, name: &str, value: Value) {
        if !self.current_section_mut().mod_var(name, value.clone()) {
            if !self.global_section_mut().mod_var(name, value.clone()) {
                panic!("Erro: Variável \"{}\" não pode ser modificada. Não encontrada.",
                       name);
            }
        }
    }

    /// Retorna o nome da seção atual que está sendo executada
    pub fn current_section_mut<'a>(&'a mut self) -> &'a mut section::Section {
        // É garantido ter ao menos uma seção (global)
        let index = self.stack.len() - 1;
        &mut self.stack[index]
    }

    pub fn global_section_mut<'a>(&'a mut self) -> &'a mut section::Section {
        &mut self.stack[0]
    }

    pub fn global_section<'a>(&'a self) -> &'a section::Section {
        &self.stack[0]
    }

    pub fn current_section<'a>(&'a self) -> &'a section::Section {
        // É garantido ter ao menos uma seção (global)
        let index = self.stack.len() - 1;
        &self.stack[index]
    }

    /// Pega o valor de uma variavel
    fn retrieve_variable(&self, name: &str) -> Option<Value> {
        // Tenta pegar a ultima seção (atual) depois da ultima
        let backtrace = self.current_section().stack.clone();
        let cursect = self.current_section().name.clone();
        let val = match self.current_section().get_var(name) {
            Some(var) => Some(var.get_val().clone()),
            None => None,
        };
        if let Some(x) = val {
            Some(x)
        } else {
            match self.global_section().get_var(name) {
                Some(var) => Some(var.get_val().clone()),
                None => None,
            }
        }
    }

    /// Compara e coloca a comparação na VM
    pub fn compare(&mut self, left: Value, right: Value) {
        let result = comparision::compare(left, right);
        self.last_cmp = result;
    }

    /// Verifica se a ultima comparação foi a passada
    pub fn last_cmp_is(&self, cmp: comparision::Comparision) -> bool {
        use vm::comparision::Comparision::*;
        match cmp {
            Equal => self.last_cmp == Equal,
            More => self.last_cmp == More,
            Less => self.last_cmp == Less,
            NEqual => self.last_cmp != Equal,
        }
    }

    /// Carrega os elementos do programa com o que foi feito parse
    pub fn load(&mut self, units: Vec<parser::Unit>) {
        let all_sects = section::Section::load_all(units.clone());
        self.sections.extend_from_slice(&all_sects);
        self.push_global_sect(); // Empurra seçõa global pra stack
        for unit in &units {
            // Declara os globais da unidade
            for global in &unit.globals {
                let global_val = evaluate(&global.value, self);
                let global_var = variable::Variable::from(&global.identifier, global_val);
                self.declare_variable(global_var); // Empurra o global pra stack
            }
        }
    }

    /// Retorna uma nova instancia de uma VM
    pub fn new() -> VM {
        VM {
            sections: vec![],
            stack: vec![],
            last_signal: None,
            last_cmp: comparision::Comparision::NEqual,
        }
    }

    /// Verifica a existência da JAULA padrão
    pub fn has_main(&self) -> bool {
        let mut res = false;
        for sect in &self.sections {
            if sect.name == parser::kw::KW_SECT_DEFAULT {
                res = true;
                break;
            }
        }
        res
    }

    /// Inicia a execução de uma seção
    pub fn start_section(&mut self, name: &str, arguments: Vec<parameter::Parameter>) {
        let mut sect = self.get_section(name).clone();
        if sect.name == name {
            sect.rec += 1;
        }
        if !parameter::Parameter::matches(arguments.clone(), sect.args.clone()) {
            panic!("Erro chamando \"{}\", argumentos passados incompatíveis com a declaração.",
                   name);
        }
        // Empurra a seção pra stack
        self.stack.push(sect.clone());
        sect.run(self, arguments);
    }

    /// faz o retorno de uma seção, colocando o valor de retorno na stack anterior
    pub fn section_return(&mut self, ret_val: Option<Value>) {
        if let Some(val) = ret_val {
            if self.stack.len() == 1 {
                // Nenhuma seção a não ser a global
                self.global_section_mut().decl_or_mod(parser::kw::KW_RETVAL_VAR, val);
            } else {
                let cur_sect_name = self.current_section().name.clone();
                // Procura a primeira seção em que o nome seja diferente
                loop {
                    if self.current_section().name != cur_sect_name {
                        self.current_section_mut().decl_or_mod(parser::kw::KW_RETVAL_VAR, val);
                        break;
                    }
                    self.stack.pop();
                }
            }
        }
    }

    /// Declara as variaveis iniciais
    pub fn decl_initial_variables(&mut self) {
        use std::env;
        let names = vec!["CUMPADE", "UM", "BODYBUILDER"];
        let values =
            vec![Value::Str(match env::var(if cfg!(windows) { "USERNAME" } else { "USER" }) {
                     Ok(v) => v.to_uppercase(),
                     Err(_) => String::from("\"CUMPADE\""),
                 }),
                 Value::Num(1.0),
                 Value::Str(String::from("CUMPADE"))];
        for i in 0..names.len() {
            let var = variable::Variable::from(names[i], values[i].clone());
            self.declare_variable(var);
        }
    }

    fn push_global_sect(&mut self) {
        for sect in &self.sections {
            if sect.name == parser::kw::KW_SECT_GLOBAL {
                self.stack.push(sect.clone());
            }
        }
    }

    /// Executa a VM
    pub fn start(&mut self) {
        self.decl_initial_variables(); // Declara as variaveis globais padrões
        // Primeiro se inicia a seção global
        self.start_section(parser::kw::KW_SECT_GLOBAL, vec![]);
        if self.has_main() {
            // Depois inicia a seção padrão, caso ela exista
            self.start_section(parser::kw::KW_SECT_DEFAULT, vec![]);
        }
    }
}
