//! Novo modulo responsavel pela execução dos codigos birlscript
#![allow(dead_code)]

mod variable;
mod section;
mod parameter;
mod signal;
mod command;

use parser;
use value;

/// Tipo de identificador usado na VM
pub type VMID = u16;

/// Máximo possivel que uma seção pode executar recursão em si mesma
pub const MAX_RECURSION: usize = 1024;

/// Onde são executados os comandos
pub struct VM {
    /// Seções executaveis presentes na VM
    sections: Vec<section::Section>,
    /// Stack de variaveis presente na seção
    stack: Vec<variable::Variable>,
    /// Pilha de seções já executadas
    last_sections: Vec<String>,
    /// Ultimo sinal jogado à VM
    last_signal: Option<signal::Signal>,
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
        abort!("Seção \"{}\" não declarada.", name); // Se chegou até aqui, é porquê não foi encontrada
    }

    /// Declara a variavel na stack da VM
    pub fn declare_variable(&mut self, var: variable::Variable) {
        if self.stack.len() > 0 {
            // verifica, de traz pra frente, se a variavel ja foi declarada
            let mut index = self.stack.len() - 1;
            loop {
                let ref v = self.stack[index];
                if v.id == var.id {
                    abort!("Variavel \"{}\" já declarada.", var.real_id())
                }
                if index == 0 {
                    break; // Acabou a stack
                }
                index -= 1;
            }
        }
        // Variavel ainda não declarada
        self.stack.push(var);
    }

    /// Retorna o nome da seção atual que está sendo executada
    pub fn current_section<'a>(&'a self) -> &'a str {
        if self.last_sections.len() > 0 {
            let index = self.last_sections.len() - 1;
            &self.last_sections[index]
        } else {
            ""
        }
    }

    pub fn retrieve_variable(&self, name: &str, section: &str) -> value::Value {
        let varname = variable::Variable::make_name(name, section);
        // verifica de traz pra frente, variavel por variavel, se encontra alguma que é igual
        if self.stack.len() == 0 {
            abort!("Nenhuma variavel declarada. pedido: {}", name);
        }
        let mut index = self.stack.len() - 1;
        let mut found = false;
        let mut res = value::Value::Number(0.0);
        loop {
            let ref v = self.stack[index];
            if v.id == varname {
                res = v.retrieve_value(self.current_section());
                found = true;
                break;
            }
            if index == 0 {
                break; // Acabou a stack
            }
            index -= 1;
        }
        let varname = variable::Variable::make_name(name, parser::kw::KW_SECT_GLOBAL);
        if !found {
            // Não encontrada, procure do primeiro pro ultimo, incluindo somente o nome (procurando por globais)
            for v in &self.stack {
                if v.id == varname {
                    res = v.retrieve_value(self.current_section());
                    found = true;
                    break;
                }
            }
        }
        if !found {
            abort!("Variável não encontrada: \"{}\"", name)
        }
        res
    }

    /// Faz o pop das ultimas variaveis declaradas na seção
    pub fn undeclare_variables(&mut self, count: usize) {
        for _ in 0..count {
            self.stack.pop(); // Joga fora a ultima variavel declarada
        }
    }

    /// Carrega os elementos do programa com o que foi feito parse
    pub fn load(&mut self, units: Vec<parser::Unit>) {
        for unit in &units {
            // Declara os globais da unidade
            for global in &unit.globals {
                let access = if global.is_const {
                    variable::Permission::ReadOnly
                } else {
                    variable::Permission::ReadWriteAll
                };
                let global_val = value::parse_expr(&global.value, self);
                let global_var = variable::Variable::from(&global.identifier,
                                                          parser::kw::KW_SECT_GLOBAL,
                                                          global_val,
                                                          access);
                self.declare_variable(global_var); // Empurra o global pra stack
            }
        }
        let all_sects = section::Section::load_all(units);
        self.sections.extend_from_slice(&all_sects);
    }

    /// Retorna uma nova instancia de uma VM
    pub fn new() -> VM {
        VM {
            sections: vec![],
            stack: vec![],
            last_sections: vec![],
            last_signal: None,
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
        // Verifica se o ultimo nome foi dessa seção
        if self.current_section() == name {
            // Aumenta o numero do recursion
            self.get_section(name).rec += 1;
            // Se o numero de recursão ultrapassar ou chegar no maximo, aborte
            if self.get_section(name).rec >= MAX_RECURSION {
                abort!("Número máximo de recursão permitido alcançado em \"{}\"",
                       name)
            }
        } else {
            // Se não, empurre o nome da seção pra pilha
            self.last_sections.push(name.to_string());
        }
        // Não há necessidade aqui de uma referencia, então clona a seção e executa
        // (necessita?)
        let mut section = self.get_section(name).clone();
        section.run(self, arguments);
    }

    /// Declara as variaveis iniciais
    pub fn decl_initial_variables(&mut self) {
        use std::env;
        let names = vec!["CUMPADE", "UM", "BODYBUILDER"];
        let values = vec![value::Value::Str(Box::new(match env::var(if cfg!(windows) {
                              "USERNAME"
                          } else {
                              "USER"
                          }) {
                              Ok(v) => v.to_uppercase(),
                              Err(_) => String::from("\"CUMPADE\""),
                          })),
                          value::Value::Number(1.0),
                          value::Value::Str(Box::new(String::from("CUMPADE")))];
        for i in 0..names.len() {
            let var = variable::Variable::from(names[i],
                                               parser::kw::KW_SECT_GLOBAL,
                                               values[i].clone(),
                                               variable::Permission::ReadOnly);
            self.declare_variable(var);
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
