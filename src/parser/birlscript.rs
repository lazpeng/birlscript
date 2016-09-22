use parser::AST;
use parser::global::Global;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__file {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use parser::AST;
    use parser::global::Global;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_3b_22(&'input str),
        Term_22GLOBAL_22(&'input str),
        Term_22HORA_20DO_20SHOW_22(&'input str),
        Term_22HORA_20DO_20SHOW_2c_20PORRA_22(&'input str),
        Term_22valor_22(&'input str),
        Termr_23_22_5bA_2dz_5d_2b_22_23(&'input str),
        Nt____file(AST),
        Ntfile(AST),
        Ntglobal(Global),
        Ntglobal__list(Vec<Global>),
        Ntidentifier(String),
        Ntprogram__start(()),
        Ntvalue(String),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on ";", error
        0, // on "GLOBAL", error
        4, // on "HORA DO SHOW", goto 3
        5, // on "HORA DO SHOW, PORRA", goto 4
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 1
        0, // on ";", error
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 2
        0, // on ";", error
        8, // on "GLOBAL", goto 7
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 3
        0, // on ";", error
        -7, // on "GLOBAL", reduce `program_start = "HORA DO SHOW" => ActionFn(2);`
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 4
        0, // on ";", error
        -8, // on "GLOBAL", reduce `program_start = "HORA DO SHOW, PORRA" => ActionFn(3);`
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 5
        9, // on ";", goto 8
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 6
        0, // on ";", error
        8, // on "GLOBAL", goto 7
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 7
        0, // on ";", error
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        12, // on r#"[A-z]+"#, goto 11
        // State 8
        0, // on ";", error
        -5, // on "GLOBAL", reduce `global_list = global, ";" => ActionFn(5);`
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 9
        13, // on ";", goto 12
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 10
        0, // on ";", error
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        15, // on "valor", goto 14
        0, // on r#"[A-z]+"#, error
        // State 11
        0, // on ";", error
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        -6, // on "valor", reduce `identifier = r#"[A-z]+"# => ActionFn(7);`
        0, // on r#"[A-z]+"#, error
        // State 12
        0, // on ";", error
        -4, // on "GLOBAL", reduce `global_list = global_list, global, ";" => ActionFn(4);`
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 13
        -3, // on ";", reduce `global = "GLOBAL", identifier, value => ActionFn(6);`
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
        // State 14
        -9, // on ";", reduce `value = "valor" => ActionFn(8);`
        0, // on "GLOBAL", error
        0, // on "HORA DO SHOW", error
        0, // on "HORA DO SHOW, PORRA", error
        0, // on "valor", error
        0, // on r#"[A-z]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -1, // on EOF, reduce `__file = file => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `file = program_start, global_list => ActionFn(1);`
        0, // on EOF, error
        -5, // on EOF, reduce `global_list = global, ";" => ActionFn(5);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `global_list = global_list, global, ";" => ActionFn(4);`
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on __file, error
        2, // on file, goto 1
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        3, // on program_start, goto 2
        0, // on value, error
        // State 1
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 2
        0, // on __file, error
        0, // on file, error
        6, // on global, goto 5
        7, // on global_list, goto 6
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 3
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 4
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 5
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 6
        0, // on __file, error
        0, // on file, error
        10, // on global, goto 9
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 7
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        11, // on identifier, goto 10
        0, // on program_start, error
        0, // on value, error
        // State 8
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 9
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 10
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        14, // on value, goto 13
        // State 11
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 12
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 13
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
        // State 14
        0, // on __file, error
        0, // on file, error
        0, // on global, error
        0, // on global_list, error
        0, // on identifier, error
        0, // on program_start, error
        0, // on value, error
    ];
    pub fn parse_file<
        'input,
    >(
        input: &'input str,
    ) -> Result<AST, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22GLOBAL_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22HORA_20DO_20SHOW_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22HORA_20DO_20SHOW_2c_20PORRA_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22valor_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Termr_23_22_5bA_2dz_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<AST,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __file = file => ActionFn(0);
                let __sym0 = __pop_Ntfile(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // file = program_start, global_list => ActionFn(1);
                let __sym1 = __pop_Ntglobal__list(__symbols);
                let __sym0 = __pop_Ntprogram__start(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntfile(__nt), __end));
                1
            }
            3 => {
                // global = "GLOBAL", identifier, value => ActionFn(6);
                let __sym2 = __pop_Ntvalue(__symbols);
                let __sym1 = __pop_Ntidentifier(__symbols);
                let __sym0 = __pop_Term_22GLOBAL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntglobal(__nt), __end));
                2
            }
            4 => {
                // global_list = global_list, global, ";" => ActionFn(4);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_Ntglobal(__symbols);
                let __sym0 = __pop_Ntglobal__list(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntglobal__list(__nt), __end));
                3
            }
            5 => {
                // global_list = global, ";" => ActionFn(5);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_Ntglobal(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntglobal__list(__nt), __end));
                3
            }
            6 => {
                // identifier = r#"[A-z]+"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_5bA_2dz_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntidentifier(__nt), __end));
                4
            }
            7 => {
                // program_start = "HORA DO SHOW" => ActionFn(2);
                let __sym0 = __pop_Term_22HORA_20DO_20SHOW_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram__start(__nt), __end));
                5
            }
            8 => {
                // program_start = "HORA DO SHOW, PORRA" => ActionFn(3);
                let __sym0 = __pop_Term_22HORA_20DO_20SHOW_2c_20PORRA_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram__start(__nt), __end));
                5
            }
            9 => {
                // value = "valor" => ActionFn(8);
                let __sym0 = __pop_Term_22valor_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntvalue(__nt), __end));
                6
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 7 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22GLOBAL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22GLOBAL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HORA_20DO_20SHOW_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HORA_20DO_20SHOW_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HORA_20DO_20SHOW_2c_20PORRA_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HORA_20DO_20SHOW_2c_20PORRA_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22valor_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22valor_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dz_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dz_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____file<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AST, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____file(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntfile<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AST, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntfile(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntglobal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Global, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntglobal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntglobal__list<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Global>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntglobal__list(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntidentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntidentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram__start<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram__start(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntvalue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntvalue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__file::parse_file;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        59 => /* ';' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 70 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        71 => /* 'G' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        73 ... 117 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        118 => /* 'v' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        119 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 75 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        77 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 78 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        80 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 96 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 78 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        80 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 81 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        83 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 107 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        66 => /* 'B' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        67 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        66 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 110 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        66 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 18;
                            continue;
                        }
                        65 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 113 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 75 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        77 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        68 => /* 'D' */ {
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        83 => /* 'S' */ {
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        72 => /* 'H' */ {
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        87 => /* 'W' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        44 => /* ',' */ {
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 29;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        80 => /* 'P' */ {
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 31;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, AST, usize),
) -> AST
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, list, _): (usize, Vec<Global>, usize),
) -> AST
{
    AST {globals:list}
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, list, _): (usize, Vec<Global>, usize),
    (_, glb, _): (usize, Global, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Global>
{
    { let mut cpy = list.clone(); cpy.push(glb); cpy }
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, glb, _): (usize, Global, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Global>
{
    vec![glb]
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, String, usize),
    (_, val, _): (usize, String, usize),
) -> Global
{
    Global::from(id, val)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, identifier, _): (usize, &'input str, usize),
) -> String
{
    String::from(identifier)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, value, _): (usize, &'input str, usize),
) -> String
{
    String::from(value)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
