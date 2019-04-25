use std::env;

use modules::*;
use context::RawValue;

mod plugins {
    use vm::{ DynamicValue, VirtualMachine, PluginFunction };
    use parser::{ TypeKind, IntegerType };
    use vm::{ SpecialItem, SpecialItemData };

    /// Split an string into multiple parts based on another string
    /// Arguments : source : Text, splitter : Text2
    fn split_string(mut arguments : Vec<DynamicValue>, vm : &mut VirtualMachine) -> Result<Option<DynamicValue>, String>
    {
        // Well since it seems Birlscript passes arguments in the reverse order

        let splitter = match arguments.remove(0)
        {
            DynamicValue::Text(id) => {
                match vm.get_special_storage_ref().get_ref(id)
                {
                    Some(data) => match data {
                        SpecialItemData::List(_) => unreachable!(),
                        SpecialItemData::Text(s) => s.clone(),
                    }
                    None => return Err("Erro interno : Dado special com ID fornecido não existe".to_owned())
                }
            }
            _ => unreachable!()
        };
        
        // pardon me for repeating code
        let source = match arguments.remove(0)
        {
            DynamicValue::Text(id) => {
                match vm.get_special_storage_ref().get_ref(id)
                {
                    Some(data) => match data {
                        SpecialItemData::List(_) => unreachable!(),
                        SpecialItemData::Text(s) => s.clone(),
                    }
                    None => return Err("Erro interno : Dado special com ID fornecido não existe".to_owned())
                }
            }
            _ => unreachable!()
        };

        let result = source.split(splitter.as_str()).map(|e| {
            Box::new(DynamicValue::Text(vm.get_special_storage_mut().add(SpecialItemData::Text(e.to_owned()))))
        }).collect::<Vec<Box<DynamicValue>>>();

        let result_id = {
            vm.get_special_storage_mut().add(SpecialItemData::List(result))
        };

        Ok(Some(DynamicValue::List(result_id)))
    }

    pub fn get_plugins() -> Vec<(String, Vec<TypeKind>, PluginFunction)> {
        vec![("DIVIDE TEXTO".to_owned(), vec![TypeKind::Text, TypeKind::Text], split_string)]
    }
}

pub fn module_standard_library() -> Module {
    let mut module = Module::new("PADRÃO".to_owned());

    let vars = vec!
    [
        ("UM".to_owned(), RawValue::Integer(1)),
        ("CUMPADE".to_owned(), RawValue::Text(env::var("USER").unwrap_or("CUMPADE".to_owned()))),
        ("FRANGO".to_owned(), RawValue::Null),
    ];

    for (name, value) in vars {
        module.global_variables.push(GlobalVariable::new(name, value, false));
    }

    for (name, params, func) in plugins::get_plugins() {
        module.plugin_functions.push(Plugin::new(name, params, func));
    }

    module
}
