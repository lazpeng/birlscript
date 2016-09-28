
use parser::command::Command;

#[derive(Clone)]
/// Represents a expected parameter in a function call. Has a id and a type
pub struct Parameter {
    /// Identifier of the parameter
    id: String,
    /// Parameter's type
    tp: String,
}

impl Parameter {
    /// Creates a Parameter object using the passed information
    pub fn from(id: String, tp: String) -> Parameter {
        Parameter { id: id, tp: tp }
    }
}

#[derive(Clone)]
/// A function that can be defined from the source code or the program itself.
/// It can't represent a native callback
pub struct Function {
    /// Name of the UD Function
    name: String,
    /// List of expected parameters
    params: Vec<Parameter>,
    /// List of commands inside the UDF
    cmds: Vec<Command>,
}

impl Function {
    /// Creates a new UDF, from a name and a list of commands and parameters
    pub fn from(name: String, parameters: Vec<Parameter>, commands: Vec<Command>) -> Function {
        Function {
            name: name,
            params: parameters,
            cmds: commands,
        }
    }
}
