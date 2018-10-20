//! The virtual machine runs code (DUH)

use birl::parser::IntegerType;
use birl::context::{ FunctionEntry, BIRL_RET_VAL_VAR_ID };

use std::io::{ Write, stdout };
use std::fmt::{ Display, self };

type StringStorageID = u64;

const MAIN_STACK_SIZE : usize = 256;

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
            Comparision::Equal =>    write!(f, "Igual"),
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

struct StringEntry {
    id : u64,
    content : String,
}

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
        let current_id = self.last_id + 1;

        let entry = StringEntry {
            content : content.to_owned(),
            id : current_id,
        };

        self.entries.push(entry);

        self.last_id = current_id;

        current_id
    }
}

struct RuntimeVariable {
    id : u64,
    address : usize,
}

pub struct FunctionFrame {
    id : u64,
    stack : Vec<DynamicValue>,
    program_counter : u64,
    last_comparision : Option<Comparision>,
    runtime_vars : Vec<RuntimeVariable>,
    next_address : usize,
    string_storage : StringStorage,
    ready : bool,
    skip_level : u32,
}

impl FunctionFrame {
    pub fn new(id : u64) -> FunctionFrame {
        FunctionFrame {
            id,
            stack : vec![],
            program_counter : 0,
            last_comparision : None,
            runtime_vars : vec![],
            next_address : 0usize,
            string_storage : StringStorage::new(),
            ready : false,
            skip_level : 0,
        }
    }

    fn get_address_of(&self, id : u64) -> Option<usize> {
        for v in &self.runtime_vars {
            if v.id == id {
                return Some(v.address);
            }
        }

        None
    }

    fn create_runtime_var(&mut self, id : u64) -> Result<(), String> {
        let address = self.next_address;
        self.next_address += 1;

        self.runtime_vars.push(RuntimeVariable { id, address });
        self.stack.push(DynamicValue::Null);

        Ok(())
    }
}

pub struct VirtualMachine {
    has_quit : bool,
    main_stack : [DynamicValue; MAIN_STACK_SIZE],
    main_stack_top : usize,
    main_storage : StringStorage,
    callstack : Vec<FunctionFrame>,
    is_interactive : bool,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            has_quit : false,
            main_stack : [DynamicValue::Null; MAIN_STACK_SIZE],
            main_stack_top : 0,
            main_storage : StringStorage::new(),
            callstack : vec![FunctionFrame::new(0)],
            is_interactive : false,
        }
    }

    pub fn set_interactive(&mut self) {
        self.is_interactive = true;
    }

    pub fn get_current_skip_level(&self) -> u32 {
        match self.get_last_ready_ref() {
            Some(f) => f.skip_level,
            None => 0,
        }
    }

    fn get_last_ready_ref(&self) -> Option<&FunctionFrame> {
        let len = self.callstack.len();

        if len == 0 {
            None
        } else if len == 1 {
            if self.callstack[0].ready {
                Some(&self.callstack[0])
            } else {
                None
            }
        } else {
            for i in (len - 1)..0 {
                if self.callstack[i].ready {
                    return Some(&self.callstack[i])
                }
            }

            None
        }
    }

    fn get_last_ready_mut(&mut self) -> Option<&mut FunctionFrame> {
        let len = self.callstack.len();

        if len == 0 {
            None
        } else if len == 1 {
            if self.callstack[0].ready {
                Some(&mut self.callstack[0])
            } else {
                None
            }
        } else {
            for i in (len - 1)..0 {
                if self.callstack[i].ready {
                    return Some(&mut self.callstack[i])
                }
            }

            None
        }
    }

    pub fn get_current_id(&self) -> Option<u64> {
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

    pub fn flush_stdout(&self) {
        let mut out = stdout();

        match out.flush() {
            Ok(_) => {}
            Err(_) => {}
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
        if ! self.is_interactive {
            return Err("PrintDebug on non-interactive mode".to_owned());
        }

        let top = match self.get_main_top() {
            Some(t) => t,
            None => return Err("MainPrintDebug : Main stack is empty".to_owned()),
        };

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
                        unimplemented!()
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

    fn write_main_top_to(&mut self, stack_index : usize, id : u64) -> Result<(), String> {
        if self.callstack.len() <= stack_index {
            return Err(format!("Index inválido : {}", stack_index));
        }

        if self.main_stack_top == 0 {
            return Err("Main stack underflow".to_owned());
        }

        self.main_stack_top -= 1;
        let val = self.main_stack[self.main_stack_top];

        let frame = &mut self.callstack[stack_index];

        let addr = match frame.get_address_of(id) {
            Some(a) => a,
            None => {
                match frame.create_runtime_var(id) {
                    Ok(_) => {
                        match frame.get_address_of(id) {
                            Some(i) => i,
                            None => return Err(format!("ID {} not found", id)),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        if frame.stack.len() <= addr {
            return Err("Endereço inválido pra stack".to_owned());
        }

        frame.stack[addr] = val;

        Ok(())
    }

    fn increase_skip_level(&mut self) {
        match self.get_last_ready_mut() {
            Some(f) => f.skip_level += 1,
            None => {}
        }
    }

    fn decrease_skip_level(&mut self) {
        match self.get_last_ready_mut() {
            Some(f) => f.skip_level -= 1,
            None => {}
        }
    }

    fn create_runtime_var(&mut self, id : u64) -> Result<(), String> {
        if self.callstack.is_empty() {
            return Err("CreateVar : Callstack tá vazia. Possível erro interno".to_owned());
        }

        let frame = self.callstack.last_mut().unwrap();

        frame.create_runtime_var(id)
    }

    pub fn run(&mut self, inst : &Instruction) -> Result<(), String> {
        if self.get_current_skip_level() > 0 {
            if let &Instruction::EndExecuteIf = inst {
                self.decrease_skip_level();
            }

            return Ok(());
        }

        match inst {
            Instruction::EndExecuteIf => {},
            Instruction::PushMainInt(i) => {
                let dyn = DynamicValue::Integer(*i);

                match self.push_main(dyn) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::PushMainNum(n) => {
                let dyn = DynamicValue::Number(*n);

                match self.push_main(dyn) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::PushMainStr(str) => {
                let id = self.main_storage.add(str.as_str());

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
                    DynamicValue::Integer(i) => print!("{}", i),
                    DynamicValue::Number(n) => print!("{}", n),
                    DynamicValue::Text(t) => {
                        let t = match self.main_storage.get_ref(t) {
                            Some(t) => t,
                            None => return Err(format!("MainPrint : Não foi encontrado text com ID {}", t)),
                        };

                        print!("\"{}\"", t);
                    }
                    DynamicValue::Null => print!("<Null>"),
                }
            }
            Instruction::MainPrintDebug => {
                match self.print_debug_main_top() {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::PrintNewLine => {
                println!();
            }
            Instruction::Quit => {
                self.has_quit = true;
            }
            Instruction::FlushStdout => {
                self.flush_stdout();
            }
            Instruction::ReadVarWithId(id) => {
                if self.callstack.is_empty() {
                    return Err("Callstack vazia. Possível erro interno".to_owned());
                }

                let val = {

                    let frame = self.callstack.last().unwrap();

                    let addr = match frame.get_address_of(*id) {
                        Some(a) => a,
                        None => return Err(format!("Endereço pra ID {} não encontrado.", *id)),
                    };

                    if frame.stack.len() <= addr {
                        return Err("Erro : Endereço pra variável é inválido".to_owned());
                    }

                    frame.stack[addr]
                };

                match self.push_main(val) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::WriteToVarWithId(id) => {
                if self.is_interactive {
                    match self.print_debug_main_top() {
                        Ok(_) => {}
                        Err(e) => return Err(e)
                    }
                }

                let len = self.callstack.len();
                match self.write_main_top_to(len - 1, *id) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::ReadGlobalVarWithId(id) => {
                if self.callstack.is_empty() {
                    return Err("Callstack vazia. Possível erro interno".to_owned());
                }

                let val = {

                    let frame = self.callstack.last().unwrap();

                    let addr = match frame.get_address_of(*id) {
                        Some(a) => a,
                        None => return Err(format!("Endereço pra ID {} não encontrado.", *id)),
                    };

                    if frame.stack.len() <= addr {
                        return Err("Erro : Endereço pra variável é inválido".to_owned());
                    }

                    frame.stack[addr]
                };

                match self.push_main(val) {
                    Some(_) => {}
                    None => return Err("Main stack overflow".to_owned()),
                }
            }
            Instruction::WriteToGlobalVarWithId(id) => {
                if self.is_interactive {
                    match self.print_debug_main_top() {
                        Ok(_) => {}
                        Err(e) => return Err(e)
                    }
                }

                match self.write_main_top_to(0, *id) {
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

                if self.is_interactive {
                    println!("(Comparação) : {}", result);
                }
            }
            Instruction::Return => {
                if self.callstack.len() < 2 {
                    return Err("Return : Não é possível retornar na função global".to_owned());
                }

                let len = self.callstack.len();
                match self.write_main_top_to(len - 2, BIRL_RET_VAL_VAR_ID) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            Instruction::CreateVarWithId(id) => {
                match self.create_runtime_var(*id) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::ExecuteIfEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::Equal {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::ExecuteIfNotEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last == Comparision::Equal {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::ExecuteIfGreater => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::MoreThan {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::ExecuteIfGreaterOrEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::Equal && last != Comparision::MoreThan {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::ExecuteIfLess => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::LessThan {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::ExecuteIfLessOrEqual => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level();
                } else {
                    let last = match self.get_last_comparision() {
                        Ok(c) => c,
                        Err(e) => return Err(e)
                    };

                    if last != Comparision::LessThan && last != Comparision::Equal {
                        self.increase_skip_level();
                    }
                }
            }
            Instruction::MakeNewFrame => {
                // Add a new, not ready frame to the callstack
            }
            Instruction::SetLastFrameReady => {
                // Set the last frame to ready

                if ! self.callstack.is_empty() {
                    self.callstack.last_mut().unwrap().ready = true;
                }
            }
        }

        Ok(())
    }

    pub fn has_quit(&self) -> bool {
        self.has_quit
    }
}

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
    ReadVarWithId(u64),
    ReadGlobalVarWithId(u64),
    WriteToVarWithId(u64),
    WriteToGlobalVarWithId(u64),
    CreateVarWithId(u64),
    CompareMainTop,
    Return,
    EndExecuteIf,
    ExecuteIfEqual,
    ExecuteIfNotEqual,
    ExecuteIfGreater,
    ExecuteIfGreaterOrEqual,
    ExecuteIfLess,
    ExecuteIfLessOrEqual,
    MakeNewFrame,
    SetLastFrameReady,
}