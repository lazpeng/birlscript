//! Module with text manipulation functions

use parser::TypeKind;
use vm::PluginFunction;

mod plugins
{
    use vm::{ DynamicValue, SpecialItemData, VirtualMachine };
    use parser::IntegerType;

    /// Split an string into multiple parts based on another string
    /// Arguments : source : Text, splitter : Text2
    pub fn split_string(mut arguments : Vec<DynamicValue>, vm : &mut VirtualMachine) -> Result<Option<DynamicValue>, String> {
        // Well since it seems Birlscript passes arguments in the reverse order

        let result = {
            let mut get_str_arg = ||
                {
                    match arguments.remove(0) {
                        DynamicValue::Text(id) => {
                            match vm.get_special_storage_ref().get_data_ref(id)
                                {
                                    Some(data) => match data {
                                        SpecialItemData::List(_) => unreachable!(),
                                        SpecialItemData::Text(s) => Ok(s),
                                    }
                                    None => Err("Erro interno : Dado special com ID fornecido não existe".to_owned())
                                }
                        }
                        _ => unreachable!()
                    }
                };

            let splitter = get_str_arg()?;

            let source = get_str_arg()?;

            source.split(splitter).map(|e| e.to_owned()).collect::<Vec<String>>()
        };

        let result_id = {
            let storage = vm.get_special_storage_mut();

            let elements = result.into_iter().map(|e| Box::new(DynamicValue::Text(storage.add(SpecialItemData::Text(e), 0u64)))).collect::<Vec<Box<DynamicValue>>>();

            storage.add(SpecialItemData::List(elements), 0u64)
        };

        Ok(Some(DynamicValue::List(result_id)))
    }

    /// Returns the length of the given string
    /// Arguments : String
    pub fn get_string_length(mut arguments : Vec<DynamicValue>, vm : &mut VirtualMachine) -> Result<Option<DynamicValue>, String> {
        let length = {
            match arguments.remove(0) {
                DynamicValue::Text(id) => {
                    match vm.get_special_storage_ref().get_data_ref(id) {
                        Some(data) => match data {
                            &SpecialItemData::Text(ref t) => t.len(),
                            _ => return Err("".to_owned())
                        }
                        None => return Err("ID special inválida".to_owned())
                    }
                }
                _ => unreachable!()
            }
        };

        Ok(Some(DynamicValue::Integer(length as IntegerType)))
    }
}

pub fn get_plugins() -> Vec<(String, Vec<TypeKind>, PluginFunction)>
{
    vec!
    [
        ("DIVIDE TEXTO".to_owned(), vec![TypeKind::Text, TypeKind::Text], plugins::split_string),
        ("TAMANHO DO TEXTO".to_owned(), vec![TypeKind::Text], plugins::get_string_length),
    ]
}