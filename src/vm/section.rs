use nparser::AST;
use nparser::command::Command;
use nparser::function::{ExpectedParameter, Function};
use vm::parameter;
use vm::command;
use vm::signal;
use vm::variable::Variable;
use eval::Value;

use super::*;

/// Ambiente da seção
#[derive(Clone)]
pub struct Section {
    /// Comandos a serem executados
    pub commands: Vec<Command>,
    /// Argumentos esperados para a seção
    pub args: Vec<ExpectedParameter>,
    /// Nome identificador da seção
    pub name: String,
    /// Identificador da seção
    id: VMID,
    /// Contador de recursão, máximo definido em mod.rs
    pub rec: usize,
    /// Stack de variaveis
    pub stack: Vec<Variable>,
}

impl Section {
    /// Cria uma nova seção baseada na seção que foi feito parsing original
    pub fn from_parser(func: &Function, id: VMID) -> Section {
        Section {
            commands: func.get_commands().clone(),
            args: func.get_parameters().clone(),
            name: func.get_identifier().to_owned(),
            id: id,
            rec: 1,
            stack: vec![],
        }
    }

    /// Faz conversão de várias seções de dentro de um Unit pra um vetor de Sections
    pub fn from_ast(ast: AST, vmid: &mut VMID) -> Vec<Section> {
        let mut res: Vec<Section> = vec![];
        for ref parsed in ast.declared_functions() {
            res.push(Section::from_parser(parsed, *vmid));
            *vmid += 1;
        }
        res
    }

    /// Faz conversão de todas as units para um só vetor de seções
    pub fn load_all(asts: Vec<AST>) -> Vec<Section> {
        let mut res: Vec<Section> = vec![];
        let mut vmid: VMID = 0;
        for ast in asts {
            let tmp = Section::from_ast(ast, &mut vmid);
            res.extend_from_slice(&tmp);
        }
        res
    }

    /// Retrieves a variable from the section's stack
    pub fn get_var<'a>(&'a self, name: &str) -> Option<&'a Variable> {
        if self.stack.is_empty() {
            panic!("Stack vazia");
        }
        for v in &self.stack {
            if v.get_id() == name {
                return Some(v);
            }
        }
        return None;
    }

    pub fn get_var_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut Variable> {
        if self.stack.is_empty() {
            panic!("Stack vazia");
        }
        for v in &mut self.stack {
            if v.get_id() == name {
                return Some(v);
            }
        }
        return None;
    }

    pub fn decl_var(&mut self, var: Variable) {
        for v in &self.stack {
            if v.get_id() == var.get_id() {
                panic!("Variavel \"{}\" já declarada.", v.get_id());
            }
        }
        self.stack.push(var);
    }

    pub fn mod_var(&mut self, name: &str, value: Value) -> bool {
        match self.get_var_mut(name) {
            Some(x) => {
                x.modify(value);
                true
            }
            None => false,
        }
    }

    pub fn decl_or_mod(&mut self, name: &str, value: Value) {
        for v in &mut self.stack {
            if v.get_id() == name {
                v.modify(value.clone());
            }
        }
        let var = Variable::from(name, value.clone());
        self.stack.push(var);
    }

    /// Roda a seção atual
    pub fn run(&mut self, vm: &mut VM, args: Vec<parameter::Parameter>) {
        if self.rec >= MAX_RECURSION {
            panic!("Máximo de recursão alcançado");
        }
        use std::process;
        use vm::variable::Variable;
        use eval::Value;
        if !parameter::Parameter::matches(args.clone(), self.args.clone()) {
            panic!("Os argumentos para \"{}\" tem tipos diferentes ou uma quantidade diferente do \
                    esperado foi passado.",
                   self.name)
        }
        let jaula = Variable::from("JAULA", Value::Str(self.name.clone()));
        vm.declare_variable(jaula); // Declara JAULA na seção atual
        for arg in args {
            vm.declare_variable(arg.var); // Declara os argumentos passados
        }
        for command in &self.commands {
            let signal = command::run(command.clone(), vm);
            vm.last_signal = signal.clone();
            match signal {
                Some(sig) => {
                    match sig {
                        signal::Signal::Quit(code) => process::exit(code),
                        signal::Signal::Return => break,
                    }
                }
                None => {}
            }
        }
    }
}
