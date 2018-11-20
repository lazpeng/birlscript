//! The virtual machine runs code (DUH)

use parser::{ TypeKind, IntegerType };
use context::RawValue;

use std::io::{ Write, BufRead };
use std::fmt::{ Display, self };

const STACK_DEFAULT_SIZE : usize = 128;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparision {
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
}

#[derive(Debug, Clone, Copy)]
pub enum ComparisionRequest {
    Equal,
    NotEqual,
    Less, LessOrEqual,
    More, MoreOrEqual,
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
    Text(u64),
    List(u64),
    Null,
}

#[derive(Debug)]
enum SpecialItemData {
    Text(String),
    List(Vec<Box<DynamicValue>>)
}

#[derive(Debug)]
struct SpecialItem {
    data : SpecialItemData,
    item_id : u64,
}

#[derive(Debug)]
struct SpecialStorage {
    items : Vec<SpecialItem>,
    next_item_id : u64,
}

impl SpecialStorage {
    fn new() -> SpecialStorage {
        SpecialStorage {
            items : vec![],
            next_item_id : 0,
        }
    }

    fn add(&mut self, data : SpecialItemData) -> u64 {
        let item_id = self.next_item_id;
        self.next_item_id += 1;

        let item = SpecialItem {
            data,
            item_id
        };

        self.items.push(item);

        item_id
    }

    fn remove_top(&mut self, num : usize) -> Result<(), String> {
        if self.items.len() < num {
            Err("remove_top : Número pra remover é maior que o número guardado".to_owned())
        } else {

            for _ in 0..num {
                let _ = self.items.pop().expect("Impossível. Erro no pop em remove_top");
            }

            Ok(())
        }
    }

    fn get_ref(&self, id : u64) -> Option<&SpecialItemData> {
        for e in &self.items {
            if e.item_id == id {
                return Some(&e.data);
            }
        }

        None
    }

    fn get_mut(&mut self, id : u64) -> Option<&mut SpecialItemData> {
        for e in &mut self.items {
            if e.item_id == id {
                return Some(&mut e.data);
            }
        }

        None
    }
}

#[derive(Debug)]
struct LoopLabel {
    start_pc : usize,
    index_address : Option<usize>,
    stepping : DynamicValue,
}

impl LoopLabel {
    fn new(start_pc : usize) -> LoopLabel {
        LoopLabel {
            start_pc,
            index_address : None,
            stepping : DynamicValue::Null,
        }
    }
}

#[derive(Debug)]
pub struct FunctionFrame {
    id : usize,
    stack : Vec<DynamicValue>,
    program_counter : usize,
    last_comparision : Option<Comparision>,
    next_address : usize,
    ready : bool,
    skip_level : u32,
    stack_size : usize,
    // Number of special items allocated
    num_special_items : usize,
    label_stack : Vec<LoopLabel>,
}

impl FunctionFrame {
    pub fn new(id : usize, stack_size : usize) -> FunctionFrame {
        FunctionFrame {
            id,
            stack : vec![DynamicValue::Null; stack_size],
            program_counter : 0,
            last_comparision : None,
            next_address : 0usize,
            ready : false,
            skip_level : 0,
            stack_size,
            label_stack : vec![],
            num_special_items : 0,
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

struct Registers {
    math_a : DynamicValue,
    math_b : DynamicValue,
    intermediate : DynamicValue,
    first_operation : bool,
    secondary : DynamicValue,
}

impl Registers {
    fn default() -> Registers {
        Registers {
            math_a : DynamicValue::Null,
            math_b : DynamicValue::Null,
            secondary : DynamicValue::Null,
            intermediate : DynamicValue::Null,
            first_operation : false,
        }
    }
}

pub struct VirtualMachine {
    registers : Registers,
    has_quit : bool,
    stack_size : usize,
    callstack : Vec<FunctionFrame>,
    stdout: Option<Box<Write>>,
    stdin:  Option<Box<BufRead>>,
    code : Vec<Vec<Instruction>>,
    next_code_index : usize,
    is_interactive : bool,
    special_storage : SpecialStorage,
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
            registers : Registers::default(),
            stack_size : STACK_DEFAULT_SIZE,
            callstack : vec![],
            stdout: None,
            stdin: None,
            code : vec![],
            next_code_index : 0,
            is_interactive : false,
            special_storage : SpecialStorage::new(),
        }
    }

    fn add_special_item(&mut self, frame_index : usize, data : SpecialItemData) -> Result<u64, String> {
        if self.callstack.len() <= frame_index {
            return Err("add_special_item : Index é inválido".to_owned());
        }

        self.callstack[frame_index].num_special_items += 1;

        Ok(self.special_storage.add(data))
    }

    fn raw_to_dynamic(&mut self, val : RawValue) -> Result<DynamicValue, String> {
        match val {
            RawValue::Text(t) => {
                let parent_index = match self.get_last_ready_index() {
                    Some(s) => s,
                    None => 0,
                };

                let id = match self.add_special_item(parent_index, SpecialItemData::Text(t)) {
                    Ok(id) => id,
                    Err(e) => return Err(e)
                };

                Ok(DynamicValue::Text(id))
            },
            RawValue::Number(n) => Ok(DynamicValue::Number(n)),
            RawValue::Integer(i) => Ok(DynamicValue::Integer(i)),
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

                        let mut result = String::new();

                        {
                            let left_v = match self.special_storage.get_ref(r_t) {
                                Some(s) => match s {
                                    &SpecialItemData::Text(ref s) => s,
                                    _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                                },
                                None => return Err(format!("Add w/ Text : Id {} não encontrada.", r_t))
                            };

                            // remove right node
                            let right_v = match self.special_storage.get_ref(l_t) {
                                Some(s) => match s {
                                    &SpecialItemData::Text(ref s) => s,
                                    _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                                },
                                None => return Err(format!("Add w/ Text : Id {} não encontrada.", l_t))
                            };

                            if self.registers.first_operation {
                                result.push_str(right_v);
                                result.push_str(left_v);

                                self.registers.first_operation = false;
                            } else {
                                result.push_str(left_v);
                                result.push_str(right_v);

                            }
                        }

                        let parent_index = match self.get_last_ready_index() {
                            Some(idx) => idx,
                            None => return Err("Nenhuma função em execução".to_owned())
                        };

                        let id = match self.add_special_item(parent_index, SpecialItemData::Text(result)) {
                            Ok(id) => id,
                            Err(e) => return Err(e)
                        };

                        Ok(DynamicValue::Text(id))
                    }
                    _ => return Err("Incompatível. Não deveria chegar aqui.".to_owned()),
                }
            }
            DynamicValue::List(left_id) => {
                match right {
                    DynamicValue::List(right_id) => {
                        // We must create a new list, add elements from left, then right, then return it

                        let mut data = vec![];

                        match self.special_storage.get_ref(left_id) {
                            Some(SpecialItemData::List(ref contents)) => {
                                for item in contents {
                                    data.push(item.clone());
                                }
                            }
                            Some(_) => return Err("Erro interno : DynamicValue é uma lista, mas o valor guardado não".to_owned()),
                            None => return Err("Erro interno : ID inválida pra lista".to_owned())
                        }

                        match self.special_storage.get_ref(right_id) {
                            Some(SpecialItemData::List(ref contents)) => {
                                for item in contents {
                                    data.push(item.clone());
                                }
                            }
                            Some(_) => return Err("Erro interno : DynamicValue é uma lista, mas o valor guardado não".to_owned()),
                            None => return Err("Erro interno : ID inválida pra lista".to_owned())
                        }

                        let index = match self.get_last_ready_index() {
                            Some(i) => i,
                            None => return Err("Nenhuma função em execução".to_owned())
                        };

                        let id = self.add_special_item(index, SpecialItemData::List(data))?;

                        Ok(DynamicValue::List(id))
                    }
                    _ => return Err("Operação não suportada entre Listas e outros valores".to_owned())
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
            DynamicValue::List(_) => return Err("Operação não suportada em listas".to_owned())
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
            DynamicValue::List(_) => return Err("Operação não suportada em listas".to_owned())
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
            DynamicValue::List(_) => return Err("Operação não suportada em listas".to_owned())
        }
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
                        let ltext = match self.special_storage.get_ref(l_t) {
                            Some(s) => match s {
                                &SpecialItemData::Text(ref s) => s,
                                _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                            },
                            None => return Err(format!("Erro : TextID não encontrada : {}", l_t)),
                        };

                        let rtext = match self.special_storage.get_ref(r_t) {
                            Some(s) => match s {
                                &SpecialItemData::Text(ref s) => s,
                                _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                            },
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
            DynamicValue::List(left_id) => {
                match right {
                    DynamicValue::List(right_id) => {
                        let left_list = match self.special_storage.get_ref(left_id) {
                            Some(SpecialItemData::List(ref list)) => list.clone(),
                            Some(_) => return Err("Erro interno : DynamicValue é uma lista mas o item guardado não".to_owned()),
                            None => return Err("ID não existe".to_owned())
                        };

                        let right_list = match self.special_storage.get_ref(right_id) {
                            Some(SpecialItemData::List(ref list)) => list.clone(),
                            Some(_) => return Err("Erro interno : DynamicValue é uma lista mas o item guardado não".to_owned()),
                            None => return Err("ID não existe".to_owned())
                        };

                        if left_list.len() != right_list.len() {
                            Comparision::NotEqual
                        } else {

                            for i in 0..left_list.len() {
                                match self.compare(*left_list[i].clone(), *right_list[i].clone()) {
                                    Ok(Comparision::Equal) => {},
                                    Ok(_) => return Ok(Comparision::NotEqual),
                                    Err(e) => return Err(e)
                                }
                            }

                            Comparision::Equal
                        }
                    }
                    _ => Comparision::NotEqual,
                }
            }
            DynamicValue::Null => {
                match right {
                    DynamicValue::Null => Comparision::Equal,
                    _ => Comparision::NotEqual,
                }
            }
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

    fn write_to(&mut self, val : DynamicValue, stack_index : usize, address : usize) -> Result<(), String> {
        if self.callstack.len() <= stack_index {
            return Err(format!("Index inválido : {}", stack_index));
        }

        let frame = &mut self.callstack[stack_index];

        if frame.stack.len() <= address {
            return Err("Endereço inválido pra stack".to_owned());
        }

        frame.stack[address] = val;

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

    fn read_from_id(&mut self, index : usize, address : usize) -> Result<DynamicValue, String> {
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

        Ok(val)
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
                let s = match self.special_storage.get_ref(t) {
                    Some(s) => match s {
                        &SpecialItemData::Text(ref s) => s,
                        _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                    },
                    None => return Err("Invalid string ID".to_owned()),
                };

                Ok(s.clone())
            }
            DynamicValue::Integer(i) => Ok(format!("{}", i)),
            DynamicValue::Number(n) => Ok(format!("{}", n)),
            DynamicValue::Null => Ok(String::from("<Null>")),
            DynamicValue::List(id) => {
                let list = match self.special_storage.get_ref(id) {
                    Some(SpecialItemData::List(ref list)) => list.clone(),
                    Some(_) => return Err("Erro interno : DynamicValue é uma lista, item interno não".to_owned()),
                    None => return Err("ID inválida pra lista".to_owned())
                };
                
                let mut result = String::from("[ ");
                let mut first = true;

                for item in list {
                    if !first {
                        result.push_str(", ");
                    } else {
                        first = false;
                    }

                    let s = self.conv_to_string(*item.clone())?;

                    result.push_str(s.as_str());
                }

                result.push_str(" ]");

                Ok(result)
            }
        }
    }

    fn conv_to_int(&mut self, val : DynamicValue) -> Result<IntegerType, String> {
        match val {
            DynamicValue::Text(t) => {
                let text = match self.special_storage.get_ref(t) {
                    Some(s) => match s {
                        &SpecialItemData::Text(ref s) => s,
                        _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                    },
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
            DynamicValue::List(_) => return Err("Não é possível converter uma lista pra inteiro".to_owned())
        }
    }

    fn conv_to_num(&mut self, val : DynamicValue) -> Result<f64, String> {
        match val {
            DynamicValue::Text(t) => {
                let text = match self.special_storage.get_ref(t) {
                    Some(s) => match s {
                        &SpecialItemData::Text(ref s) => s,
                        _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                    },
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
            DynamicValue::List(_) => return Err("Não é possível converter uma lista pra número".to_owned())
        }
    }

    fn last_comparision_matches(&self, req : ComparisionRequest) -> Result<bool, String> {
        let last = match self.get_last_comparision() {
            Ok(c) => c,
            Err(e) => return Err(e)
        };

        match req {
            ComparisionRequest::Equal => Ok(last == Comparision::Equal),
            ComparisionRequest::NotEqual => Ok(last != Comparision::Equal),
            ComparisionRequest::Less => Ok(last == Comparision::LessThan),
            ComparisionRequest::LessOrEqual => Ok(last == Comparision::LessThan || last == Comparision::Equal),
            ComparisionRequest::More => Ok(last == Comparision::MoreThan),
            ComparisionRequest::MoreOrEqual => Ok(last == Comparision::MoreThan || last == Comparision::Equal),
        }
    }

    pub fn set_stack_size(&mut self, size : usize) {
        self.stack_size = size;
    }

    fn set_current_pc(&mut self, pc : usize) -> Result<(), String> {
        match self.get_last_ready_mut() {
            Some(f) => f.program_counter = pc,
            None => return Err("Nenhuma função em execução".to_owned())
        };

        Ok(())
    }

    pub fn run(&mut self, inst : Instruction) -> Result<ExecutionStatus, String> {
        //println!("{:?}", inst);
        if self.get_current_skip_level() > 0 {
            if let Instruction::EndConditionalBlock = inst {
                self.decrease_skip_level()?;
            }

            return Ok(ExecutionStatus::Normal);
        }

        match inst {
            Instruction::EndConditionalBlock => {},
            Instruction::PrintMathBDebug => {
                match self.registers.math_b {
                    DynamicValue::Integer(i) => vm_write!(self.stdout, "(Integer) {}\n", i)?,
                    DynamicValue::Number(n) => vm_write!(self.stdout, "(Number) {}\n", n)?,
                    DynamicValue::Text(t) => {
                        let t = match self.special_storage.get_ref(t) {
                            Some(s) => match s {
                                &SpecialItemData::Text(ref s) => s,
                                _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                            },
                            None => return Err(format!("MainPrint : Não foi encontrado text com ID {}", t)),
                        };

                        vm_write!(self.stdout, "(Text) \"{}\"\n", t)?
                    }
                    DynamicValue::Null => vm_write!(self.stdout, "<Null>")?,
                    DynamicValue::List(id) => {
                        let string = match self.conv_to_string(DynamicValue::List(id)) {
                            Ok(s) => s,
                            Err(e) => return Err(e)
                        };
                        vm_write!(self.stdout, "{}\n", string)?;
                    }
                }

                self.flush_stdout();
            }
            Instruction::PrintMathB => {
                match self.registers.math_b {
                    DynamicValue::Integer(i) => vm_write!(self.stdout, "{}", i)?,
                    DynamicValue::Number(n) => vm_write!(self.stdout, "{}", n)?,
                    DynamicValue::Text(t) => {
                        let t = match self.special_storage.get_ref(t) {
                            Some(s) => match s {
                                &SpecialItemData::Text(ref s) => s,
                                _ => return Err(format!("Erro interno : DynamicValue é texto, mas o id aponta pra outra coisa"))
                            },
                            None => return Err(format!("MainPrint : Não foi encontrado text com ID {}", t)),
                        };

                        vm_write!(self.stdout, "{}", t)?
                    }
                    DynamicValue::List(id) => {
                        let string = match self.conv_to_string(DynamicValue::List(id)) {
                            Ok(s) => s,
                            Err(e) => return Err(e)
                        };
                        vm_write!(self.stdout, "(Lista) {}", string)?;
                    }
                    DynamicValue::Null => vm_write!(self.stdout, "<Null>")?,
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
            Instruction::Compare => {
                let result = match self.compare(self.registers.math_a, self.registers.math_b) {
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
                    self.has_quit = true;

                    return Ok(ExecutionStatus::Quit);
                }

                match self.callstack.pop() {
                    Some(frame) => self.special_storage.remove_top(frame.num_special_items)?,
                    None => return Err("Erro no return : Nenhuma função em execução".to_owned())
                }

                let index = self.callstack.len() - 1;
                let val = self.registers.math_b;
                match self.write_to(val, index, 0) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }

                return Ok(ExecutionStatus::Returned);
            }
            Instruction::ExecuteIf(req) => {
                if self.get_current_skip_level() > 0 {
                    self.increase_skip_level()?;
                } else {
                    if ! self.last_comparision_matches(req)? {
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
            Instruction::AssertMathBCompatible(kind) => {
                let v = self.registers.math_b;

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
                    DynamicValue::List(_) => {
                        if kind == TypeKind::List {
                            // Ok
                        } else {
                            return Err("Tipo incompatível : Lista".to_owned());
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

                    let last_index = line.len() - 1;
                    line.remove(last_index);

                    Some(line)
                } else { None };

                let parent_index = match self.get_last_ready_index() {
                    Some(s) => s,
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                if let Some(line) = line {
                    let id = match self.add_special_item(parent_index, SpecialItemData::Text(line)) {
                        Ok(id) => id,
                        Err(e) => return Err(e)
                    };

                    self.registers.intermediate = DynamicValue::Text(id);
                }
            }
            Instruction::ConvertToNum => {
                let val = self.registers.math_b;

                let v = match self.conv_to_num(val) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = DynamicValue::Number(v);
            }
            Instruction::ConvertToInt => {
                let val = self.registers.math_b;

                let v = match self.conv_to_int(val) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = DynamicValue::Integer(v);
            }
            Instruction::ConvertToString => {
                let val = self.registers.math_b;

                let id = if let DynamicValue::Text(id) = val {
                    id
                } else {
                    let v = match self.conv_to_string(val) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    };

                    let parent_index = match self.get_last_ready_index() {
                        Some(s) => s,
                        None => return Err("Nenhuma função em execução".to_owned())
                    };

                    match self.add_special_item(parent_index, SpecialItemData::Text(v)) {
                        Ok(id) => id,
                        Err(e) => return Err(e)
                    }
                };

                self.registers.math_b = DynamicValue::Text(id);
            }
            Instruction::PushValMathA(val) => {
                match self.raw_to_dynamic(val) {
                    Ok(v) => self.registers.math_a = v,
                    Err(e) => return Err(e)
                }
            }
            Instruction::PushValMathB(val) => {
                match self.raw_to_dynamic(val) {
                    Ok(v) => self.registers.math_b = v,
                    Err(e) => return Err(e)
                }
            }
            Instruction::PushIntermediateToA => {
                self.registers.math_a = self.registers.intermediate;
            }
            Instruction::PushIntermediateToB => {
                self.registers.math_b = self.registers.intermediate;
            }
            Instruction::ReadGlobalVarFrom(addr) => {
                let val = match self.read_from_id(0, addr) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.intermediate = val;
            }
            Instruction::WriteGlobalVarTo(addr) => {
                let index = 0;
                let val = self.registers.math_b;

                match self.write_to(val, index, addr) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            Instruction::ReadVarFrom(addr) => {
                let index = match self.get_last_ready_index() {
                    Some(i) => i,
                    None => return Err("Nenhuma função pronta em execução".to_owned()),
                };

                let val = match self.read_from_id(index, addr) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.intermediate = val;
            }
            Instruction::WriteVarTo(addr) => {
                let index = match self.get_last_ready_index() {
                    Some(i) => i,
                    None => return Err("Nenhuma função pronta em execução".to_owned()),
                };

                let val = self.registers.math_b;

                match self.write_to(val, index, addr) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }
            Instruction::WriteVarToLast(addr) => {
                let index = self.callstack.len() - 1;
                let val = self.registers.math_b;

                match self.write_to(val, index, addr) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            Instruction::Add => {
                let left = self.registers.math_a;
                let right = self.registers.math_b;
                let res = match self.add_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = res;
            }
            Instruction::Mul => {
                let left = self.registers.math_a;
                let right = self.registers.math_b;
                let res = match self.mul_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = res;
            }
            Instruction::Div => {
                let left = self.registers.math_a;
                let right = self.registers.math_b;
                let res = match self.div_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = res;
            }
            Instruction::Sub => {
                let left = self.registers.math_a;
                let right = self.registers.math_b;
                let res = match self.sub_values(left, right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = res;
            }
            Instruction::SwapMath => {
                let tmp = self.registers.math_b;
                self.registers.math_b = self.registers.math_a;
                self.registers.math_a = tmp;
            }
            Instruction::ClearMath => {
                self.registers.math_a = DynamicValue::Null;
                self.registers.math_b = DynamicValue::Null;
                self.registers.intermediate = DynamicValue::Null;
            }
            Instruction::AddLoopLabel => {
                let next_pc = match self.get_current_pc() {
                    Some(p) => p,
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                match self.get_last_ready_mut() {
                    Some(f) => f.label_stack.push(LoopLabel::new(next_pc)),
                    None => return Err("Nenhuma função em execução".to_owned())
                }
            }
            Instruction::RestoreLoopLabel => {
                let (mut address, mut step) = (None, DynamicValue::Null);

                let pc = match self.get_last_ready_ref() {
                    Some(f) => {
                        let label = match f.label_stack.last() {
                            Some(l) => l,
                            None => return Err("Restore : Nenhuma label disponível".to_owned())
                        };

                        if let Some(addr) = label.index_address {
                            address = Some(addr);
                            step = label.stepping;
                        }

                        label.start_pc
                    }
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                self.set_current_pc(pc)?;

                if let Some(address) = address {
                    let index = match self.get_last_ready_index() {
                        Some(i) => i,
                        None => return Err("Nenhuma função pronta em execução".to_owned()),
                    };

                    let current = self.read_from_id(index, address)?;

                    let result = self.add_values(current, step)?;

                    match self.write_to(result, index, address) {
                        Ok(_) => {}
                        Err(e) => return Err(e)
                    }
                }
            }
            Instruction::PopLoopLabel => {
                match self.get_last_ready_mut() {
                    Some(f) => {
                        match f.label_stack.pop() {
                            Some(_) => {}
                            None => return Err("Não havia nenhuma label pra remover".to_owned())
                        }
                    }
                    None => return Err("Nenhuma função em execução".to_owned())
                }
            }
            Instruction::RegisterIncrementOnRestore(address) => {
                // Since this instruction is right after AddLabel, this is going to be executed each iteration
                // and since we don't want that, we'll also increment the PC on the label

                let stepping = self.registers.math_b;

                match self.get_last_ready_mut() {
                    Some(s) => match s.label_stack.last_mut() {
                        Some(l) => {
                            l.stepping = stepping;
                            l.index_address = Some(address);
                            // As explained above
                            l.start_pc += 1;
                        }
                        None => return Err("Função atual não tem nenhuma label".to_owned()),
                    }
                    None => return Err("Nenhuma função em execução".to_owned())
                };
            }
            Instruction::SetFirstExpressionOperation => {
                self.registers.first_operation = true;
            }
            Instruction::MakeNewList => {
                let index = match self.get_last_ready_index() {
                    Some(i) => i,
                    None => return Err("Nenhuma função em execução".to_owned())
                };

                let data = match self.add_special_item(index, SpecialItemData::List(vec![])) {
                    Ok(d) => d,
                    Err(e) => return Err(e)
                };

                self.registers.math_b = DynamicValue::List(data);
            }
            Instruction::IndexList => {
                unimplemented!()
            }
            Instruction::AddToListAtIndex => {
                let index = if let DynamicValue::Integer(val) = self.registers.secondary {
                    Some(val)
                } else {
                    None
                };

                let value = self.registers.math_b;

                let list_id = if let DynamicValue::List(id) = self.registers.intermediate {
                    id
                } else {
                    return Err(format!("AddListToIndex : A variável não é uma lista"));
                };

                let list = match self.special_storage.get_mut(list_id) {
                    Some(l) => match l {
                        SpecialItemData::List(ref mut list) => list,
                        _ => return Err("Item especial com a ID passada não é uma lista".to_owned())
                    }
                    None => return Err("ID da lista não encontrada".to_owned())
                };

                if let Some(i) = index {
                    if i as usize >= list.len() {
                        list.push(Box::new(value));
                    } else {
                        list.insert(i as usize, Box::new(value));
                    }
                } else {
                    list.push(Box::new(value));
                }
            }
            Instruction::ClearSecondary => {
                self.registers.secondary = DynamicValue::Null;
            }
            Instruction::PushMathBToSeconday => {
                let val = self.registers.math_b;
                self.registers.secondary = val;
            }
            Instruction::RemoveFromListAtIndex => {
                unimplemented!()
            }
            Instruction::QueryListSize => {
                let id = if let DynamicValue::List(id) = self.registers.intermediate {
                    id
                } else {
                    return Err("QueryListSize : Variável não é uma lista".to_owned());
                };

                let list = match self.special_storage.get_ref(id) {
                    Some(l) => match l {
                        SpecialItemData::List(l) => l,
                        _ => return Err("Erro interno : ID não aponta pra uma lista".to_owned())
                    }
                    None => return Err("Não encontrado item com a ID passada".to_owned())
                };

                let val = DynamicValue::Integer(list.len() as IntegerType);

                self.registers.math_b = val;
            }
        }

        Ok(ExecutionStatus::Normal)
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    PrintMathB,
    PrintMathBDebug,
    PrintNewLine,
    FlushStdout,
    Quit,
    Compare,
    Return,
    EndConditionalBlock,
    ExecuteIf(ComparisionRequest),
    MakeNewFrame(usize),
    SetLastFrameReady,
    // For use when pushing arguments for a function. Check if the value on the top of the main stack
    // has a compatible type
    AssertMathBCompatible(TypeKind),
    // Get a line of input and put it at the top of the main stack
    ReadInput,
    // Turn the main stack top into string
    ConvertToString,
    // Turn the main stack top into num
    ConvertToNum,
    // Turn the main stack top into int
    ConvertToInt,
    PushValMathA(RawValue),
    PushValMathB(RawValue),
    PushIntermediateToA,
    PushIntermediateToB,
    PushMathBToSeconday,
    ClearSecondary,
    // Values are read to the intermediate register
    ReadGlobalVarFrom(usize),
    // When writing, values are read from the math b register
    WriteGlobalVarTo(usize),
    ReadVarFrom(usize),
    WriteVarTo(usize),
    WriteVarToLast(usize),
    SwapMath,
    ClearMath,
    Add,
    Mul,
    Div,
    Sub,
    /// Saves the current PC so when the loop ends it can return to it's beginning
    AddLoopLabel,
    /// Return to a previous saved loop label
    RestoreLoopLabel,
    /// Remove a previously saved label
    PopLoopLabel,
    /// Retrieve the increment value from MathB and write it on every Restore to the specified address
    RegisterIncrementOnRestore(usize),
    /// Set the register to denote this is the first operation on the expression
    SetFirstExpressionOperation,
    /// Create a new list and put the result at MathB
    MakeNewList,
    /// Index a list with the ID from the intermediate register and the index from MathB, and put the result in MathB
    IndexList,
    /// Add the result in MathB to the list in the intermediate register, using the index at the secondary register
    /// if the secondary register is Null, the element is placed on the back of the list
    AddToListAtIndex,
    /// Remove the element at the index located in MathB from the list in the intermediate register
    RemoveFromListAtIndex,
    /// Query the list from the intermediate address and write its size to the MathB
    QueryListSize,
}
