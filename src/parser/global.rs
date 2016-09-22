//! Module defining globals

#[derive(Clone)]
pub struct Global {
    /// Name of the global
    name: String,
    /// An expression which will be evaluated
    value: String,
}

impl Global {
    /// Creates a new global from the following information
    pub fn from(name: String, value: String) -> Global {
        Global {
            name: name,
            value: value,
        }
    }

    /// Creates a new global object, empty
    pub fn new() -> Global {
        Global {
            name: String::new(),
            value: String::new(),
        }
    }

    /// Returns the name of the global
    pub fn get_name<'a>(&'a self) -> &'a str {
        &self.name
    }

    /// Returns the value of the global
    pub fn get_value<'a>(&'a self) -> &'a str {
        &self.value
    }
}
