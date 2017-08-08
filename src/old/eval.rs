use std::str::Chars;
use std::fmt::{Display, Formatter, self};

#[cfg(target_pointer_width = "64")]
pub type NumericType = f64;

#[cfg(target_pointer_width = "32")]
pub type NumericType = f32;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Num(NumericType),
    NullOrEmpty,
}

impl Value {
    fn do_operation(&self, right: &Value, operator: Operator) -> Result<Value, String> {
        // Really really super duper dumb implementation below. This is the old backend anyway

        match self {
            &Value::Str(ref ls) => {
                if operator != Operator::Plus {
                    return Err("O único operador que strings aceitam é o +".to_owned());
                }

                let mut result = String::from(ls.as_str());

                if let &Value::Str(ref rs) = right {
                    result.push_str(rs.as_str());
                } else {
                    // Transform the right into a string, then
                    result.push_str(right.as_string().as_str());
                }

                Ok(Value::Str(result))
            }
            _ => {
                let left_val = if let &Value::Num(n) = self {
                    n
                } else {
                    0.0 // Null or empty
                };

                if let &Value::Str(_) = right {
                    return Err("TRAPEZIO DESCENDENTE não possui quaisquer operadores compatíveis com FIBRA".to_owned());
                }

                let right_val = if let &Value::Num(n) = right {
                    n
                } else {
                    0.0 // Null or empty
                };

                let result = match operator {
                    Operator::Plus => left_val + right_val,
                    Operator::Minus => left_val - right_val,
                    Operator::Multiplication => left_val * right_val,
                    Operator::Division => left_val / right_val,
                    _ => unreachable!()
                };

                Ok(Value::Num(result))
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            &Value::Str(ref sr) => sr.clone(),
            &Value::Num(num) => format!("{}", num),
            &Value::NullOrEmpty => "<nulo>".to_owned()
        }
    }

    pub fn try_parse(s: &str) -> Result<Value, String> {
        // If starts with a digit, is a number. Otherwise is a string
        if s.is_empty() {
            Ok(Value::Str(String::new()))
        } else {
            let first_char = s.chars().next().unwrap(); // Cannot fail since is not empty

            match first_char {
                '0' ... '9' => match s.parse::<NumericType>() {
                    Ok(v) => Ok(Value::Num(v)),
                    Err(e) => return Err(format!("{}", e)),
                }
                _ => Ok(Value::Str(s.to_owned()))
            }
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            &Value::Str(_) => ValueType::Str,
            &Value::Num(_) => ValueType::Num,
            &Value::NullOrEmpty => ValueType::NullOrEmpty,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Value::Num(x) => write!(f, "{}", x),
            &Value::Str(ref x) => write!(f, "{}", x),
            &Value::NullOrEmpty => write!(f, "<nulo>"), // Empty value
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Num,
    Str,
    NullOrEmpty,
}

const VALUETYPE_STR: &str = "FIBRA";
const VALUETYPE_NUM: &str = "TRAPEZIO DESCENDENTE";

impl ValueType {
    pub fn try_parse(expr: &str) -> Option<ValueType> {
        match expr.trim() {
            VALUETYPE_STR => Some(ValueType::Str),
            VALUETYPE_NUM => Some(ValueType::Num),
            _ => None,
        }
    }
}

pub trait ValueQuery {
    fn query(&self, _: &str) -> Option<Value> {
        unimplemented!()
    }

    fn query_raw(&self, _: &str) -> Option<String> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// An operator that can be applied to one or more values
enum Operator {
    Plus,
    Minus,
    Division,
    Multiplication,
    Parenthesis,
}

impl Operator {
    fn from(c: char) -> Operator {
        match c {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '/' => Operator::Division,
            '*' => Operator::Multiplication,
            '(' => Operator::Parenthesis,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
/// An element of a expression
enum ExprElem {
    Value(Value),
    Operator(Operator),
}

// The base if for "should the parser quit when it finds the end of the input or a )?"
fn eval_scope<Query>(input: &mut Chars, query_func: &Query, base: bool) -> Result<Value, String>
    where Query: ValueQuery
{
    // Somehow this abomination works.

    // The stack is where the final evaluation is made
    let mut stack: Vec<ExprElem> = vec![];
    let mut last_value = Value::NullOrEmpty;
    let mut last_operator = Operator::Plus;
    let mut last_value_contents = String::new();
    // If the next value we find should be evaluated
    let mut eval_next = false;
    // Was the last value a variable?
    let mut was_last_variable = false;
    // Are we parsing a string?
    let mut is_inside_string = false;
    // Considering that we're currently inside a string, was the last character a escape?
    let mut last_character_was_escape = false;

    loop {
        if let Some(c) = input.next() {
            if c == '\n' {
                if !base {
                    return Err("O input terminou dentro de algum parentesis".to_owned());
                } else {
                    break;
                }
            } else if c == '\"' {
                if is_inside_string {
                    if last_character_was_escape {
                        last_character_was_escape = false;
                        last_value_contents.push(c);
                    } else {
                        is_inside_string = false;
                    }
                } else {
                    is_inside_string = true;
                }
            } else {
                match c {
                    '\\' if is_inside_string => {
                        if last_character_was_escape {
                            last_value_contents.push(c);
                            last_character_was_escape = false;
                        } else {
                            last_character_was_escape = true;
                        }
                    }

                    _ if is_inside_string => {
                        // Already checked for ", just push it
                        last_value_contents.push(c);
                    }

                    ')' => {
                        if !base {
                            break;
                        } else {
                            return Err("Parêntesis de fechamento sem nenhum aberto".to_owned());
                        }
                    }

                    '(' => {
                        if eval_next {
                            eval_next = false;

                            let paren = match eval_scope(input, query_func, false) {
                                Ok(v) => v,
                                Err(e) => return Err(e),
                            };

                            let result = match last_value.do_operation(&paren, last_operator) {
                                Ok(v) => v,
                                Err(e) => return Err(e)
                            };

                            last_value = Value::NullOrEmpty;

                            stack.push(ExprElem::Value(result));
                        } else {
                            let paren = match eval_scope(input, query_func, false) {
                                Ok(v) => v,
                                Err(e) => return Err(e)
                            };

                            stack.push(ExprElem::Value(paren));
                        }
                    }

                    ' ' if last_value_contents.is_empty() || !is_inside_string => {}

                    '+' | '-' | '/' | '*' => {
                        // Separator. Evaluate the value and do something that depends on the operator

                        if last_value_contents.is_empty() {
                            // Just push the operator and last value into the stack
                            if last_value != Value::NullOrEmpty {
                                stack.push(ExprElem::Value(last_value));
                                last_value = Value::NullOrEmpty;
                            }
                            stack.push(ExprElem::Operator(Operator::from(c)));
                            continue;
                        }

                        let mut current_val = if was_last_variable {
                            was_last_variable = false;
                            match query_func.query(&last_value_contents) {
                                Some(v) => v,
                                None => return Err(format!("A variável {} não foi encontrada", last_value_contents))
                            }
                        } else {
                            match Value::try_parse(&last_value_contents) {
                                Ok(v) => v,
                                Err(e) => return Err(e),
                            }
                        };

                        last_value_contents.clear();

                        if eval_next {
                            eval_next = false;

                            current_val = match last_value.do_operation(&current_val, last_operator) {
                                Ok(v) => v,
                                Err(e) => return Err(e)
                            };

                            last_value = Value::NullOrEmpty;
                        }

                        if c == ' ' { continue; }

                        // If the operator has low precedence, push it to the stack and that's all
                        // However, if the operator was high precedence, set it the value to the last one
                        // and set the flag to evaluate whatever is the next value, then push the result to the stack

                        if c == '+' || c == '-' {
                            stack.push(ExprElem::Value(current_val));
                            stack.push(ExprElem::Operator(Operator::from(c)));
                        } else {
                            last_value = current_val;
                            last_operator = Operator::from(c);
                            eval_next = true;
                        }
                    }

                    '_' | 'a' ... 'z' | 'A' ... 'Z' if last_value_contents.is_empty() => {
                        was_last_variable = true;
                        last_value_contents.push(c);
                    }

                    _ => {
                        last_value_contents.push(c);
                    }
                }
            }
        } else {
            if !base {
                return Err("O input terminou dentro de algum parentesis".to_owned());
            }
            break;
        }
    }

    // Check if no value was left behind
    if !last_value_contents.is_empty() {
        let res = if was_last_variable {
            match query_func.query(&last_value_contents) {
                Some(v) => v,
                None => return Err(format!("Variável {} não encontrada", last_value_contents))
            }
        } else {
            match Value::try_parse(&last_value_contents) {
                Ok(v) => v,
                Err(e) => return Err(e),
            }
        };

        if eval_next {
            stack.push(match last_value.do_operation(&res, last_operator) {
                Ok(v) => ExprElem::Value(v),
                Err(e) => return Err(e)
            });
        } else {
            stack.push(ExprElem::Value(res));
        }
    }

    // Evaluate the stack
    if stack.is_empty() {
        Err("Stack vazia".to_owned())
    } else if stack.len() == 1 {
        if let ExprElem::Value(v) = stack.pop().unwrap() {
            Ok(v)
        } else {
            Err("Stack só tem um elemento e é um operador :/".to_owned())
        }
    } else {
        let mut skip_first = false;
        let mut result = match &stack[0] {
            &ExprElem::Operator(_) => Value::NullOrEmpty,
            &ExprElem::Value(ref v) => {
                skip_first = true;
                v.clone()
            }
        };

        let mut stack_iter = stack.iter();

        if skip_first {
            let _ = stack_iter.next();
        }

        loop {
            // At each iteration, get one operator and one value (for the right) and operate with the result
            if let Some(elem) = stack_iter.next() {
                if let &ExprElem::Operator(op) = elem {
                    match stack_iter.next() {
                        Some(elem) => {
                            if let &ExprElem::Value(ref v) = elem {
                                result = match result.do_operation(v, op) {
                                    Ok(v) => v,
                                    Err(e) => return Err(e),
                                };
                            } else {
                                return Err(format!("Dois valores seguidos numa expressão. stack {:?}", stack));
                            }
                        }
                        None => break,
                    }
                } else {
                    return Err(format!("Dois valores seguidos numa expressão. stack {:?}", stack));
                }
            } else {
                break;
            }
        }

        println!("result is {:?}", result);

        Ok(result)
    }
}

/// Evaluate a expression and return the value wrapper in the Value enum
pub fn evaluate<Query>(expression: &str, query_func: &Query) -> Result<Value, String>
    where Query: ValueQuery
{
    let mut chars = expression.chars();
    eval_scope(&mut chars, query_func, true)
}
