use parser;
use super::parameter;
use super::command;
use super::signal;

use super::*;

/// Ambiente da seção
#[derive(Clone)]
pub struct Section {
    /// Comandos a serem executados
    pub commands: Vec<parser::Command>,
    /// Argumentos esperados para a seção
    args: Vec<parser::ExpectedParameter>,
    /// Nome identificador da seção
    pub name: String,
    /// Identificador da seção
    id: VMID,
    /// Contador de recursão, máximo definido acima
    pub rec: usize,
}

impl Section {
    /// Cria uma nova seção baseada na seção que foi feito parsing original
    pub fn from_parser(section: parser::Section, id: VMID) -> Section {
        Section {
            commands: section.lines.clone(),
            args: section.param_list.clone(),
            name: section.name.clone(),
            id: id,
            rec: 1,
        }
    }

    /// Faz conversão de várias seções de dentro de um Unit pra um vetor de Sections
    pub fn from_unit(unit: parser::Unit, vmid: &mut VMID) -> Vec<Section> {
        let mut res: Vec<Section> = vec![];
        for parsed in unit.sects {
            res.push(Section::from_parser(parsed, *vmid));
            *vmid += 1;
        }
        res
    }

    /// Faz conversão de todas as units para um só vetor de seções
    pub fn load_all(units: Vec<parser::Unit>) -> Vec<Section> {
        let mut res: Vec<Section> = vec![];
        let mut vmid: VMID = 0;
        for unit in units {
            let tmp = Section::from_unit(unit, &mut vmid);
            res.extend_from_slice(&tmp);
        }
        res
    }

    /// Roda a seção atual
    pub fn run(&mut self, vm: &mut VM, args: Vec<parameter::Parameter>) {
        if !parameter::Parameter::matches(args, self.args.clone()) {
            abort!("Os argumentos para \"{}\" tem tipos diferentes ou uma quantidade diferente do \
                    esperado foi passado.",
                   self.name)
        }
        use std::process;
        for command in &self.commands {
            let signal = command::run(command.clone(), vm);
            vm.last_signal = signal.clone();
            match signal {
                Some(sig) => {
                    match sig {
                        signal::Signal::Quit(code) => process::exit(code),
                        _ => {}
                    }
                }
                None => {}
            }
        }
    }
}
