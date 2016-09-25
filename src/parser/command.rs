
#[derive(Clone)]
/// An executable command that holds zero or more values and perform one or more actions
pub enum Command {
    /// Prints a value to the standard stream
    Print(String),
    /// Prints a value to the standard stream, followed by a newline
    Println(String),
}

impl Command {
    /// Used by the parser generator. Given a line, parses the command and it's arguments
    pub fn try_parse(src: &str) -> Result<Command, &str> {
        unimplemented!()
    }
}
