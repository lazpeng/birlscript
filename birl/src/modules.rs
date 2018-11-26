use context::RawValue;
use vm::PluginFunction;
use parser::{ Command, TypeKind, FunctionParameter };

pub struct GlobalVariable {
    pub name : String,
    pub writeable : bool,
    pub value : RawValue,
}

impl GlobalVariable {
    pub fn new(name : String, value : RawValue, writeable : bool) -> GlobalVariable {
        GlobalVariable {
            name,
            value,
            writeable
        }
    }
}

pub struct Plugin {
    pub name : String,
    pub parameters : Vec<TypeKind>,
    pub func : PluginFunction,
}

impl Plugin {
    pub fn new(name : String, parameters : Vec<TypeKind>, func : PluginFunction) -> Plugin {
        Plugin {
            name,
            parameters,
            func
        }
    }
}

pub struct SourceFunction {
    pub name : String,
    pub parameters : Vec<FunctionParameter>,
    pub body : Vec<Command>,
}

impl SourceFunction {
    pub fn new(name : String, parameters : Vec<FunctionParameter>, body : Vec<Command>) -> SourceFunction {
        SourceFunction {
            name,
            parameters,
            body
        }
    }
}

pub struct Module {
    pub global_variables : Vec<GlobalVariable>,
    pub plugin_functions : Vec<Plugin>,
    pub source_functions : Vec<SourceFunction>,
    pub name : String,
}

impl Module {
    pub fn new(name : String) -> Module {
        Module {
            global_variables : vec![],
            plugin_functions : vec![],
            source_functions : vec![],
            name,
        }
    }
}
