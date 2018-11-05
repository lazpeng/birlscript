//! The virtual machine runs code (DUH)

use parser::{ TypeKind, IntegerType };

use std::io::{ Write, BufRead };
use std::fmt::{ Display, self };

type StringStorageID = u64;

const MAIN_STACK_SIZE : usize = 256;
const STACK_DEFAULT_SIZE : usize = 128;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparision {
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
}

impl Display for Comparision {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Comparision::Equal    => write!(f, "Igual"),
            Comparision::NotEqual => write!(f, "Diferente"),
            Comparision::LessThan => write!(f, "Menor"),
            Comparision::MoreThan => write!(f, "Maior"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DynamicValue {
    Integer(IntegerType),
    Number(f64),
    Text(StringStorageID),
    Null,
}

#[derive(Debug)]
struct StringEntry {
    id : u64,
    content : String,
}

#[derive(Debug)]
struct StringStorage {
    entries : Vec<StringEntry>,
    last_id : u64,
}

impl StringStorage {
    fn new() -> StringStorage {
        StringStorage {
            entries: vec![],
            last_id : 0,
        }
    }

    fn get_ref(&self, id : u64) -> Option<&str> {
        for e in &self.entries {
            if e.id == id {
                return Some(e.content.as_str());
            }
        }

        None
    }

    fn get_mut(&mut self, id : u64) -> Option<&mut String> {
        for e in &mut self.entries {
            if e.id == id {
                return Some(&mut e.content);
            }
        }

        None
    }

    fn get(&mut self, id : u64) -> Option<String> {
        for i in 0..self.entries.len() {
            let cid = self.entries[i].id;

            if cid == id {
                let entry = self.entries.remove(i);

                return Some(entry.content);
            }
        }

        None
    }

    fn add(&mut self, content : &str) -> u64 {
        self.add_string(content.to_owned())
    }

    fn add_string(&mut self, content : String) -> u64 {
        let id = self.last_id + 1;

        let entry = StringEntry {
            content,
            id,
        };

        self.entries.push(entry);

        self.last_id = id;

        id
    }
}

#[derive(Debug)]
pub struct FunctionFrame {
    id : usize,
    stack : Vec<DynamicValue>,
    program_counter : usize,
    last_comparision : Option<Comparision>,
    next_address : usize,
    string_storage : StringStorage,
    ready : bool,
    skip_level : u32,
    stack_size : usize,
}

impl FunctionFrame {
    pub fn new(id : usize, stack_size : usize) -> FunctionFrame {
        FunctionFrame {
            id,
            stack : vec![DynamicValue::Null; stack_size],
            program_counter : 0,
            last_comparision : None,
            next_address : 0usize,
            string_storage : StringStorage::new(),
            ready : false,
            skip_level : 0,
            stack_size,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExecutionStatus {
    Normal,
    Quit,
    Returned,
    Halt,
}

pub struct VirtualMachine {
    has_quit : bool,
    main_stack : [DynamicValue; MAIN_STACK_SIZE],
    stack_size : usize,
    main_stack_top : usize,
    main_storage : StringStorage,
    callstack : Vec<FunctionFrame>,
    stdout: Option<Box<Write>>,
    stdin:  Option<Box<BufRead>>,
    code : Vec<Vec<Instruction>>,
    next_code_index : usize,
    is_interactive : bool,
}

macro_rules! vm_write{
    ($out:expr,$($arg:tt)*) => ({
        if let Some(output) = $out.as_mut(){
            write!(output, $($arg)*)
                .map_err(|what| format!("Deu pra escrever não cumpade: {:?}", what))
        }else{
            Ok(())
        }
    })
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            has_quit : false,
            main_stack : [DynamicValue::Null; MAIN_STACK_SIZE],
            main_stack_top : 0,
            main_storage : StringStorage::new(),
            stack_size : STACK_DEFAULT_SIZE,
            callstack : vec![],
            stdout: None,
            stdin: None,
            code : vec![],
            next_code_index : 0,
            is_interactive : false,
        }
    }

    pub fn set_interactive_mode(&mut self) {
        self.is_interactive = true;
    }

    pub fn execute_next_instruction(&mut self) -> Result<ExecutionStatus, String> {
        if self.callstack.is_empty() {
            return Err("Nenhuma função em execução".to_owned());
        }

        let pc = match self.get_current_pc() {
            Some(p) => p,
            None => return Err("Nenhuma função em execução".to_owned()),
        };

        let id = match self.get_current_id() {
            Some(i) => i,
            None => return Err("Nenhuma função em execução".to_owned())
        };

        if self.code.len() <= id {
            return Err("ID atual pra função é inválida".to_owned());
        }

        let instruction = {
            let code = &self.code[id];

            if code.len() <= pc {
                if self.callstack.len() == 1 && self.is_interactive {
                    return Ok(ExecutionStatus::Halt);
                } else {
                    Instruction::Return
                }
            } else {
                code[pc].clone()
            }
        };

        match self.increment_pc() {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        self.run(instruction)
    }

    pub fn set_stdout(&mut self, write: Option<Box<Write>>) -> Option<Box<Write>>{
        use std::mem;
        mem::replace(&mut self.stdout, write)
    }

    pub fn set_stdin(&mut self, read: Option<Box<BufRead>>) -> Option<Box<BufRead>>{
        use std::mem;
        mem::replace(&mut self.stdin, read)
    } 

    pub fn get_current_skip_level(&self) -> u32 {
        match self.get_last_ready_ref() {
            Some(f) => f.skip_level,
            None => 0,
        }
    }

    fn get_last_ready_ref(&self) -> Option<&FunctionFrame> {
        let callstack = &self.callstack;
        for frame in callstack.into_iter().rev() {
            if frame.ready {
                return Some(frame);
            }
        }
        None
    }

    pub fn get_last_ready_mut(&mut self) -> Option<&mut FunctionFrame> {
        let callstack = &mut self.callstack;
        for frame in callstack.into_iter().rev() {
            if frame.ready {
                return Some(frame);
            }
        }
        None
    }

    fn get_current_id(&self) -> Option<usize> {
        if self.callstack.is_empty() {
            None
        } else {
            match self.get_last_ready_ref() {
                Some(f) => Some(f.id),
                None => None,
            }
        }
    }

    fn get_main_top(&self) -> Option<DynamicValue> {
        if self.main_stack_top == 0 {
            return None;
        }
        Some(self.main_stack[self.main_stack_top - 1])
    }

    pub fn get_next_code_id(&self) -> usize {
        self.next_code_index
    }

    pub fn get_code_for(&mut self, id : usize) -> Option<&mut Vec<Instruction>> {
        if self.code.len() <= id {
            None
        } else {
            Some(&mut self.code[id])
        }
    }

    pub fn add_new_code(&mut self) -> usize {
        let id = self.next_code_index;
        self.next_code_index += 1;
        self.code.push(vec![]);

        id
    }

    fn pop_main(&mut self) -> Option<DynamicValue> {
        if self.main_stack_top == 0 {
            return None;
        }
        let d = match self.get_main_top() {
            Some(v) => v,
            None => return None,
        };
        self.main_stack_top -= 1;

        Some(d)
    }

    fn push_main(&mut self, v : DynamicValue) -> Option<()> {
        if self.main_stack_top >= MAIN_STACK_SIZE {
            return None;
        }

        self.main_stack[self.main_stack_top] = v;
        self.main_stack_top += 1;

        Some(())
    }

    pub fn flush_stdout(&mut self) {
        if let Some(ref mut out) = self.stdout.as_mut(){
            match out.flush() {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    fn is_compatible(left : DynamicValue, right : DynamicValue) -> bool {
        match left {
            DynamicValue::Text(_) => {
                if let DynamicValue::Text(_) = right {
                    true
                } else {
                    false
                }
            }
            DynamicValue::Integer(_) | DynamicValue::Number(_) => {
                match right {
                    DynamicValue::Integer(_) | DynamicValue::Number(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn add_values(&mut self, left : DynamicValue, right : DynamicValue) -> Result<DynamicValue, String> {
        if ! VirtualMachine::is_compatible(left, right) {
            return Err(format!("Add : Os valores não são compatíveis : {:?} e {:?}", left, right));
        }

        match left {
            DynamicValue::Integer(l_i) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Integer(l_i + r_i)),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number((l_i as f64) + r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Number(l_n) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Number(l_n + (r_i as f64))),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number(l_n + r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Text(l_t) => {

                match right {
                    DynamicValue::Text(r_t) => {
                        // Add right value to left node

                        let left_v = match self.main_storage.get(r_t) {
                            Some(s) => s,
                            None => return Err(format!("Add w/ Text : Id {} não encontrada.", r_t))
                        };

                        // remove right node
                        let right_v_ref = match self.main_storage.get_mut(l_t) {
                            Some(c) => c,
                            None => return Err(format!("Add w/ Text : Id {} não encontrada.", l_t))
                        };

                        right_v_ref.push_str(left_v.as_str());

                        Ok(DynamicValue::Text(l_t))
                    }
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Null => Ok(DynamicValue::Null),
        }
    }

    fn sub_values(&mut self, left : DynamicValue, right : DynamicValue) -> Result<DynamicValue, String> {
        if ! VirtualMachine::is_compatible(left, right) {
            return Err(format!("Add : Os valores não são compatíveis : {:?} e {:?}", left, right));
        }

        match left {
            DynamicValue::Integer(l_i) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Integer(l_i - r_i)),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number((l_i as f64) - r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Number(l_n) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Number(l_n - (r_i as f64))),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number(l_n - r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Text(_) => return Err("Operação inválida em texto : -".to_owned()),
            DynamicValue::Null => Ok(DynamicValue::Null),
        }
    }

    fn mul_values(&mut self, left : DynamicValue, right : DynamicValue) -> Result<DynamicValue, String> {
        if ! VirtualMachine::is_compatible(left, right) {
            return Err(format!("Add : Os valores não são compatíveis : {:?} e {:?}", left, right));
        }

        match left {
            DynamicValue::Integer(l_i) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Integer(l_i * r_i)),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number((l_i as f64) * r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Number(l_n) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Number(l_n * (r_i as f64))),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number(l_n * r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Text(_) => return Err("Operação inválida em texto : *".to_owned()),
            DynamicValue::Null => Ok(DynamicValue::Null),
        }
    }

    fn div_values(&mut self, left : DynamicValue, right : DynamicValue) -> Result<DynamicValue, String> {
        if ! VirtualMachine::is_compatible(left, right) {
            return Err(format!("Add : Os valores não são compatíveis : {:?} e {:?}", left, right));
        }

        match left {
            DynamicValue::Integer(l_i) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Integer(l_i / r_i)),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number((l_i as f64) / r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Number(l_n) => {
                match right {
                    DynamicValue::Integer(r_i) => Ok(DynamicValue::Number(l_n / (r_i as f64))),
                    DynamicValue::Number(r_n) => Ok(DynamicValue::Number(l_n / r_n)),
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::Text(_) => return Err("Operação inválida em texto : /".to_owned()),
            DynamicValue::Null => Ok(DynamicValue::Null),
        }
    }

    fn print_debug_main_top(&self) -> Result<(), String> {
        let top = match self.get_main_top() {
            Some(t) => t,
            None => return Err("MainPrintDebug : Main stack is empty".to_owned()),
        };

        print!("<");

        match top {
            DynamicValue::Integer(i) => {
                println!("(Integer) : {}", i);
            }
            DynamicValue::Number(n) => {
                println!("(Number) : {}", n);
            }
            DynamicValue::Text(t) => {
                print!("(Text) \"");

                match self.main_storage.get_ref(t) {
                    Some(ref t) => {
                        print!("{}", t);
                    }
                    None => return Err(format!("Não foi encontrado o texto com ID {}", t)),
                }

                println!("\"");
            }
            DynamicValue::Null => {
                println!("<Null>");
            }
        }

        Ok(())
    }

    fn get_last_comparision(&self) -> Result<Comparision, String> {
        if self.callstack.is_empty() {
            return Err("Callstack vazia".to_owned());
        }

        match self.callstack.last().unwrap().last_comparision {
            Some(c) => Ok(c),
            None => Err("Nenhuma comparação na função atual".to_owned())
        }
    }

    fn compare(&self, left : DynamicValue, right : DynamicValue) -> Result<Comparision, String> {
        if ! VirtualMachine::is_compatible(left, right) {
            return Err(format!("Compare : Valores incompatíveis : {:?} e {:?}", left, right));
        }

        let comp_numbers: fn(f64, f64) -> Comparision = | l, r | {
            if l == r {
                Comparision::Equal
            } else if l < r {
                Comparision::LessThan
            } else {
                Comparision::MoreThan
            }
        };

        let comp = match left {
            DynamicValue::Integer(l_i) => {
                match right {
                    DynamicValue::Integer(r_i) => {
                        if l_i == r_i {
                            Comparision::Equal
                        } else if l_i < r_i {
                            Comparision::LessThan
                        } else {
                            Comparision::MoreThan
                        }
                    }
                    DynamicValue::Number(r_n) => comp_numbers(l_i as f64, r_n),
                    _ => Comparision::NotEqual
                }
            }
            DynamicValue::Number(l_n) => {
                match right {
                    DynamicValue::Number(r_n) => {
                        comp_numbers(l_n, r_n)
                    }
                    DynamicValue::Integer(r_i) => {
                        comp_numbers(l_n, r_i as f64)
                    }
                    _ => Comparision::NotEqual,
                }
            }
            DynamicValue::Text(l_t) => {
                match right {
                    DynamicValue::Text(r_t) => {
                        let ltext = match self.main_storage.get_ref(l_t) {
                            Some(t) => t,
                            None => return Err(format!("Erro : TextID não encontrada : {}", l_t)),
                        };

                        let rtext = match self.main_storage.get_ref(r_t) {
                            Some(t) => t,
                            None => return Err(format!("Erro : TextID não encontrada : {}", r_t)),
                        };

                        let llen = ltext.len();
                        let rlen = rtext.len();

                        if llen > rlen {
                            Comparision::MoreThan
                        } else if llen < rlen {
                            Comparision::LessThan
                        } else {
                            if ltext == rtext {
                                Comparision::Equal
                            } else {
                                Comparision::NotEqual
                            }
                        }
                    }
                    _ => Comparision::NotEqual
                }
            }
            DynamicValue::Null => Comparision::NotEqual,
        };

        Ok(comp)
    }

    fn set_last_comparision(&mut self, comp : Comparision) -> Result<(), String> {
        if self.callstack.is_empty() {
            return Err("Callstack tá vazia. Provavelmente é erro interno".to_owned());
        }

        self.callstack.last_mut().unwrap().last_comparision = Some(comp);

        Ok(())
    }

    // This function doesn't search all the callstack, just the first frame
    fn get_last_ready_index(&self) -> Option<usize> {
        if self.callstack.is_empty() {
            None
        }
        else if self.callstack.len() < 2 {
            if self.callstack[0].ready {
                Some(0)
            } else {
                None
            }
        } else {
            let last = self.callstack.len() - 1;

            if self.callstack[last].ready {
                Some(last)
            } else {
                Some(last - 1)
            }
        }
    }

    fn write_main_top_to(&mut self, stack_index : usize, address : usize) -> Result<(), String> {
        if self.callstack.len() <= stack_index {
            return Err(format!("Index inválido : {}", stack_index));
        }

        if self.main_stack_top == 0 {
            return Err("Main stack underflow".to_owned());
        }

        self.main_stack_top -= 1;
        let val = self.main_stack[self.main_stack_top];

        let frame = &mut self.callstack[stack_index];

        if frame.stack.len() <= address {
            return Err("Endereço inválido pra stack".to_owned());
        }

        match val {
            DynamicValue::Text(t) => {
                let raw = match self.main_storage.get_ref(t) {
                    Some(t) => t,
                    None => return Err(format!("TextID {} é inválida.", t))
                };

                let id = frame.string_storage.add(raw);

                frame.stack[address] = DynamicValue::Text(id);
            }
            _ => frame.stack[address] = val,
        }

        Ok(())
    }

    fn increase_skip_level(&mut self) -> Result<(), String> {
        match self.get_last_ready_mut() {
            Some(f) => f.skip_level += 1,
            None => return Err("Nenhuma função ready em execução".to_owned())
        }

        Ok(())
    }

    fn decrease_skip_level(&mut self) -> Result<(), String> {
        match self.get_last_ready_mut() {
            Some(f) => f.skip_level -= 1,
            None => return Err("Nenhuma função ready em execução".to_owned())
        }

        Ok(())
    }

    fn read_from_id_to_main(&mut self, index : usize, address : usize) -> Result<(), String> {
        if self.callstack.len() < index {
            return Err(format!("Index out of bounds for read : {}", index));
        }

        let val = {

            let frame = &mut self.callstack[index];

            if frame.stack.len() <= address {
                return Err("Erro : Endereço pra variável é inválido".to_owned());
            }

            frame.stack[address]
        };

        match self.push_main(val) {
            Some(_) => {}
            None => return Err("Main stack overflow".to_owned()),
        }

        Ok(())
    }

    pub fn unset_quit(&mut self) {
        self.has_quit = false;
    }

    pub fn has_quit(&self) -> bool {
        self.has_quit
    }

    pub fn get_current_pc(&self) -> Option<usize> {
        match self.get_last_ready_ref() {
            Some(f) => Some(f.program_counter),
            None => None
        }
    }

    pub fn increment_pc(&mut self) -> Result<(), String> {
        match self.get_last_ready_mut() {
            Some(f) => f.program_counter += 1,
            None => return Err("Nenhuma função em execução".to_owned())
        }

        Ok(())
    }

    pub fn decrement_pc(&mut self) -> Result<(), String> {
        match self.get_last_ready_mut() {
            Some(f) => f.program_counter -= 1,
            None => return Err("Nenhuma função em execução".to_owned())
        }

        Ok(())

    }

    fn conv_to_string(&mut self, val : DynamicValue) -> Result<String, String> {
        match val {
            DynamicValue::Text(t) => {
                let s = match self.main_storage.get(t) {
                    Some(s) => s,
                    None => return Err("Invalid string ID".to_owned()),
                };

                Ok(s)
            }
            DynamicValue::Integer(i) => Ok(format!("{}", i)),
            DynamicValue::Number(n) => Ok(format!("{}", n)),
            DynamicValue::Null => Ok(String::from("<Null>")),
        }
    }

    fn conv_to_int(&mut self, val : DynamicValue) -> Result<IntegerType, String> {
        match val {
            DynamicValue::Text(t) => {
                let text = match self.main_storage.get(t) {
                    Some(v) => v,
                    None => return Err("Invalid text id".to_owned())
                };

                let i = match text.parse::<IntegerType>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Não foi possível converter \"{}\" pra Int", text))
                };

                Ok(i)
            }
            DynamicValue::Number(n) => Ok(n as IntegerType),
            DynamicValue::Integer(i) => Ok(i),
            DynamicValue::Null => return Err("Convert : <Null>".to_owned()),
        }
    }

    fn conv_to_num(&mut self, val : DynamicValue) -> Result<f64, String> {
        match val {
            DynamicValue::Text(t) => {
                let text = match self.main_storage.get(t) {
                    Some(v) => v,
                    None => return Err("Invalid text id".to_owned())
                };

                let n = match text.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => return Err(format!("Não foi possível converter \"{}\" pra Num", text))
                };

                Ok(n)
            }
            DynamicValue::Number(n) => Ok(n),
            DynamicValue::Integer(i) => Ok(i as f64),
            DynamicValue::Null => return Err("Convert : <Null>".to_owned()),
        }
    }

    pub fn set_stack_size(&mut self, size : usize) {
        self.stack_size = size;
    }

    pub fn run(&mut self, inst : Instruction) -> Result<ExecutionStatus, String> {
        if self.get_current_skip_level() > 0 {
            if let Instruction::EndExecuteIf = inst {
                self.decrease_skip_level()?;
            }

            return Ok(ExecutionStatus::Normal);
        }

        match inst {
            Instruction::EndExecuteIf => {},
            Instruction::PushMainInt(i) => {
                let dyn = DynamicValue::Integer(i);

                match self.push_main(dyn) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::PushMainNum(n) => {
                let dyn = DynamicValue::Number(n);

                match self.push_main(dyn) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::PushMainStr(s) => {
                let id = self.main_storage.add_string(s);

                let dyn = DynamicValue::Text(id);

                match self.push_main(dyn) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::PushNull => {
                match self.push_main(DynamicValue::Null) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::MainAdd => {
                let right = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let left = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let result = match self.add_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(result) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                };
            }
            Instruction::MainSub => {
                let right = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let left = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let result = match self.sub_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(result) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                };
            }
            Instruction::MainDiv => {
                let right = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let left = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let result = match self.div_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(result) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                };
            }
            Instruction::MainMul => {
                let right = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let left = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack is empty".to_owned())
                };

                let result = match self.mul_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(result) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                };
            }
            Instruction::MainPrint => {
                let top = match self.get_main_top() {
                    Some(t) => t,
                    None => return Err("MainPrint : Main stack is empty".to_owned()),
                };

                match top {
                    DynamicValue::Integer(i) => vm_write!(self.stdout, "{}", i)?,
                    DynamicValue::Number(n) => vm_write!(self.stdout, "{}", n)?,
                    DynamicValue::Text(t) => {
                        let t = match self.main_storage.get_ref(t) {
                            Some(t) => t,
                            None => return Err(format!("MainPrint : Não foi encontrado text com ID {}", t)),
                        };

                        vm_write!(self.stdout, "{}", t)?
                    }
                    DynamicValue::Null => vm_write!(self.stdout, "<Null>")?,
                }
            }
            Instruction::MainPrintDebug => {
                match self.print_debug_main_top() {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::PrintNewLine => {
                vm_write!(self.stdout, "\n")?
            }
            Instruction::Quit => {
                self.has_quit = true;

                return Ok(ExecutionStatus::Quit);
            }
            Instruction::FlushStdout => {
                self.flush_stdout();
            }
            Instruction::ReadVarFromAddress(address) => {
                let index = match self.get_last_ready_index() {
                    Some(i) => i,
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                match self.read_from_id_to_main(index, address) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::WriteToVarAtAddress(address) => {
                let index = match self.get_last_ready_index() {
                    Some(i) => i,
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                match self.write_main_top_to(index, address) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::WriteToLastFrameVarAtAddress(address) => {
                let len = self.callstack.len();
                match self.write_main_top_to(len - 1, address) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::ReadGlobalVarFromAddress(address) => {
                match self.read_from_id_to_main(0, address) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::WriteToGlobalVarAtAddress(address) => {
                match self.write_main_top_to(0, address) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::CompareMainTop => {
                let right = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack vazia".to_owned()),
                };

                let left = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack vazia".to_owned()),
                };

                let result = match self.compare(left, right) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };

                match self.set_last_comparision(result) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::Return => {
                if self.callstack.len() == 1 {
                    return Err("Return usado na função global".to_owned());
                }

                if self.main_stack_top == 0 {
                    self.push_main(DynamicValue::Null);
                }

                let len = self.callstack.len();
                match self.write_main_top_to(len - 2, 0) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                let _ = self.callstack.pop();

                return Ok(ExecutionStatus::Returned);
            }
            Instruction::ExecuteIfEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::Equal {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::ExecuteIfNotEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last == Comparision::Equal {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::ExecuteIfGreater => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::MoreThan {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::ExecuteIfGreaterOrEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::Equal && last != Comparision::MoreThan {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::ExecuteIfLess => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::LessThan {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::ExecuteIfLessOrEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::LessThan && last != Comparision::Equal {
                        self.increase_skip_level()?;
                    }
                }
            }
            Instruction::MakeNewFrame(id) => {
                // Add a new, not ready frame to the callstack

                let frame = FunctionFrame::new(id, self.stack_size);

                self.callstack.push(frame);
            }
            Instruction::SetLastFrameReady => {
                // Set the last frame to ready

                if ! self.callstack.is_empty() {
                    self.callstack.last_mut().unwrap().ready = true;
                } else {
                    return Err("Callstack vazia".to_owned());
                }
            }
            Instruction::AssertMainTopTypeCompatible(kind) => {
                let v = match self.get_main_top() {
                    Some(v) => v,
                    None => return Err("AssertMainTopType : Main stack vazia".to_owned()),
                };

                match v {
                    DynamicValue::Null => return Err("Tipo incompatível : Null".to_owned()),
                    DynamicValue::Text(_) => {
                        if kind == TypeKind::Text {
                            // Ok
                        } else {
                            return Err("Tipo incompatível : Texto".to_owned());
                        }
                    }
                    DynamicValue::Integer(_) => {
                        if kind == TypeKind::Integer || kind == TypeKind::Number {
                            // Ok
                        } else {
                            return Err("Tipo incompatível : Int ou Num".to_owned());
                        }
                    }
                    DynamicValue::Number(_) => {
                        if kind == TypeKind::Number {
                            // Ok
                        } else {
                            return Err("Tipo incompatível : Number".to_owned());
                        }
                    }
                }
            }
            Instruction::ReadInput => {
                let line = if let Some(ref mut input) = self.stdin.as_mut(){
                    let mut line = String::new();
                    match input.read_line(&mut line) {
                        Ok(_) => {}
                        Err(e) => return Err(format!("Erro lendo input : {:?}", e))
                    };

                    // FIXME
                    // Kinda slow, but necessary to trim the /r and /n
                    // depending on the platform I'll probably roll my own
                    // solution later tho
                    Some(line.trim().to_owned())
                } else { None };

                if let Some(line) = line{
                    let id = self.main_storage.add_string(line);
                    match self.push_main(DynamicValue::Text(id)) {
                        Some(_) => {}
                        None => return Err("Main stack overflow".to_owned())
                    };
                }
            }
            Instruction::ConvertToNum => {
                let top = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack underflow".to_owned())
                };

                let v = match self.conv_to_num(top) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(DynamicValue::Number(v)) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned())
                }
            }
            Instruction::ConvertToInt => {
                let top = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack underflow".to_owned())
                };

                let v = match self.conv_to_int(top) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match self.push_main(DynamicValue::Integer(v)) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned())
                }
            }
            Instruction::ConvertToString => {
                let top = match self.pop_main() {
                    Some(v) => v,
                    None => return Err("Main stack underflow".to_owned())
                };

                let v = match self.conv_to_string(top) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                let id = self.main_storage.add_string(v);

                match self.push_main(DynamicValue::Text(id)) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned())
                }
            }
        }

        Ok(ExecutionStatus::Normal)
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    PushMainInt(IntegerType),
    PushMainNum(f64),
    PushMainStr(String),
    PushNull,
    MainAdd,
    MainSub,
    MainDiv,
    MainMul,
    MainPrint,
    PrintNewLine,
    MainPrintDebug,
    FlushStdout,
    Quit,
    ReadVarFromAddress(usize),
    ReadGlobalVarFromAddress(usize),
    WriteToVarAtAddress(usize),
    WriteToLastFrameVarAtAddress(usize),
    WriteToGlobalVarAtAddress(usize),
    CompareMainTop,
    Return,
    EndExecuteIf,
    ExecuteIfEqual,
    ExecuteIfNotEqual,
    ExecuteIfGreater,
    ExecuteIfGreaterOrEqual,
    ExecuteIfLess,
    ExecuteIfLessOrEqual,
    MakeNewFrame(usize),
    SetLastFrameReady,
    // For use when pushing arguments for a function. Check if the value on the top of the main stack
    // has a compatible type
    AssertMainTopTypeCompatible(TypeKind),
    // Get a line of input and put it at the top of the main stack
    ReadInput,
    // Turn the main stack top into string
    ConvertToString,
    // Turn the main stack top into num
    ConvertToNum,
    // Turn the main stack top into int
    ConvertToInt,
}
