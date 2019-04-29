//! Base module for the standard library

use std::env;

use modules::*;
use context::RawValue;

mod text_manip;

fn get_global_vars() -> Vec<(String, RawValue)> {
    vec!
    [
        ("UM".to_owned(), RawValue::Integer(1)),
        ("CUMPADE".to_owned(), RawValue::Text(env::var("USER").unwrap_or("CUMPADE".to_owned()))),
        ("FRANGO".to_owned(), RawValue::Null),
    ]
}

pub fn module_standard_library() -> Module {
    let mut module = Module::new("PADRÃO".to_owned());

    let modules_plugins = vec!
    [
        text_manip::get_plugins()
    ];

    let modules_vars = vec!
    [
        get_global_vars()
    ];

    let modules_source_functions : Vec<Vec<SourceFunction>> = vec!
    [
    ];

    for vars in modules_vars {
        for (name, value) in vars {
            module.global_variables.push(GlobalVariable::new(name, value, false));
        }
    }

    for plugins in modules_plugins {
        for (name, params, func) in plugins {
            module.plugin_functions.push(Plugin::new(name, params, func));
        }
    }

    for source_functions in modules_source_functions {
        for source_func in source_functions {
            module.source_functions.push(source_func);
        }
    }

    module
}
