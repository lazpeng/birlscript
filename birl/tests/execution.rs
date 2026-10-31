extern crate birl;

use std::cell::RefCell;
use std::io::{ self, Write };
use std::rc::Rc;

use birl::context::{ Context, BIRL_GLOBAL_FUNCTION_ID };

/// Output sink shared between the test and the VM
#[derive(Clone)]
struct SharedOutput(Rc<RefCell<Vec<u8>>>);

impl Write for SharedOutput {
    fn write(&mut self, buf : &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Runs the source the same way the shell does, returning the output or the first error
fn run(source : &str) -> Result<String, String> {
    let output = SharedOutput(Rc::new(RefCell::new(vec![])));
    let mut ctx = Context::new();

    ctx.call_function_by_id(BIRL_GLOBAL_FUNCTION_ID, vec![])?;
    ctx.add_standard_library()?;
    ctx.add_source_string(source.to_owned())?;
    ctx.set_stdout(Some(Box::new(output.clone())));
    ctx.start_program()?;

    let bytes = output.0.borrow().clone();
    Ok(String::from_utf8(bytes).unwrap())
}

fn eval(expr : &str) -> Result<String, String> {
    run(&format!("CE QUER VER ISSO: {}", expr)).map(|s| s.trim_end().to_owned())
}

#[test]
fn precedence_and_associativity() {
    let cases = [
        ("10 - 3 - 2", "5"),
        ("2 * 3 - 1", "5"),
        ("100 / 10 / 2", "5"),
        ("2 * 3 + 4", "10"),
        ("4 + 2 * 3", "10"),
        ("10 - 2 * 3", "4"),
        ("2 * 3 - 4 * 5", "-14"),
        ("7 / 2", "3"),
        ("7.0 / 2", "3.5"),
        (".5 * 2", "1"),
    ];

    for &(expr, expected) in cases.iter() {
        assert_eq!(eval(expr), Ok(expected.to_owned()), "expressão: {}", expr);
    }
}

#[test]
fn parenthesis() {
    let cases = [
        ("10 - (3 + 4)", "3"),
        ("10 / (2 + 3)", "2"),
        ("(1 + 2) * (3 + 4)", "21"),
        ("2 * 3 + (4 + 5)", "15"),
        ("(10 - 4) / (1 + 2) - 1", "1"),
        ("((2))", "2"),
    ];

    for &(expr, expected) in cases.iter() {
        assert_eq!(eval(expr), Ok(expected.to_owned()), "expressão: {}", expr);
    }
}

#[test]
fn unary_operators() {
    let cases = [
        ("-5 + 2", "-3"),
        ("2 * -1", "-2"),
        ("-(2 + 3) * 2", "-10"),
        ("--3", "3"),
        ("+4", "4"),
    ];

    for &(expr, expected) in cases.iter() {
        assert_eq!(eval(expr), Ok(expected.to_owned()), "expressão: {}", expr);
    }
}

#[test]
fn text_concatenation() {
    assert_eq!(eval("\"A\" + \"B\" + \"C\""), Ok("ABC".to_owned()));
    assert_eq!(eval("\"A\" + (\"B\" + \"C\")"), Ok("ABC".to_owned()));
}

#[test]
fn variables_in_expressions() {
    let src = "JAULA SHOW
    VEM: X, 10
    VEM: Y, 3
    CE QUER VER ISSO: X - Y - 2, \" \", X / Y / 2, \" \", (X - Y) * 2
SAINDO DA JAULA";

    assert_eq!(run(src), Ok("5 1 14\n".to_owned()));
}

#[test]
fn invalid_expressions_are_errors() {
    for expr in ["(1 + 2", "1 + 2)", "1 2", "1 +", "2 * / 3", "99999999999999999999"].iter() {
        assert!(eval(expr).is_err(), "expressão deveria falhar: {}", expr);
    }
}

#[test]
fn runtime_math_errors_do_not_panic() {
    assert!(eval("1 / 0").is_err());
    assert!(eval("9223372036854775807 + 1").is_err());
    assert!(eval("-9223372036854775807 - 2").is_err());
}

#[test]
fn trailing_whitespace() {
    assert_eq!(eval("1   "), Ok("1".to_owned()));
    assert_eq!(eval("1 + 1\t"), Ok("2".to_owned()));
}

#[test]
fn standard_library_variables() {
    assert_eq!(eval("UM"), Ok("1".to_owned()));
    assert_eq!(eval("BODYBUILDER"), Ok("BAMBAM".to_owned()));
}

#[test]
fn factorial_example() {
    let src = "JAULA FATORIAL (NUMERO: BATATA DOCE)
    É ELE QUE A GENTE QUER: NUMERO, 1
    MENOR OU É MEMO:
        BIRL: 1
    FIM
    VEM: TOTAL, NUMERO
    REPETE: ATUAL, NUMERO - 1, 1, (-1)
        BORA: TOTAL, TOTAL * ATUAL
    FIM
    BIRL: TOTAL
SAINDO DA JAULA

JAULA SHOW
    E HORA DO: FATORIAL, 5
    CE QUER VER ISSO: TREZE
SAINDO DA JAULA";

    assert_eq!(run(src), Ok("120\n".to_owned()));
}
