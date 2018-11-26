use std::env;

use modules::*;
use context::RawValue;

mod plugins {
    use vm::{ DynamicValue, VirtualMachine, PluginFunction };
    use parser::{ TypeKind, IntegerType };

    pub fn get_plugins() -> Vec<(String, Vec<TypeKind>, PluginFunction)> {
        // TODO: Add functions
        vec![]
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
