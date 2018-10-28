extern crate birl;

fn main(){
	/* Print heading info. */
	eprintln!("O SHELL QUE CONSTRÓI FIBRA. VERSÃO {}", env!("CARGO_PKG_VERSION"));
	eprintln!("BIRL  © 2018, RAFAEL RODRIGUES NAKANO.");
	eprintln!("SHELL © 2018, MATHEUS BRANCO BORELLA.");
	eprintln!();

	/* Setup BIRL.
	 * The Context interpreter is not built to run interactively,
	 * by default, as it lacks much of the framework that would be
	 * needed to properly implement an interactive shell with
	 * immidiate evaluation of expresssions.
	 * 
	 * So, as a workaround, the shell cycles the interpreter manually
	 * to the completion of all instructions added by a line. To
	 * achieve that, we first call the root function, as any
	 * expression typed into the interpreted will be compiled as a
	 * root-level instruction.
	 */
	use birl::context::Context;
	let mut c = Context::new();
	
	use birl::context::BIRL_GLOBAL_FUNCTION_ID;
	c.call_function_by_id(BIRL_GLOBAL_FUNCTION_ID, vec![])
		.expect("Could not setup BIRL runtime.");

	/* Bind the Context interpreter to standard IO */
	let _ = c.set_stdin({
		use std::io;
		let reader = io::BufReader::new(io::stdin());
		Some(Box::new(reader))
	});
	let _ = c.set_stdout({
		use std::io;
		Some(Box::new(io::stdout()))
	});

	/* Enter interactive loop */
	use std::io::{stdin, BufReader, BufRead};
	let mut prompt = BufReader::new(stdin());
	loop{
		eprint!("> ");

		let mut line = String::new();
		match prompt.read_line(&mut line){
			Ok(count) => if count == 0 {
				eprintln!("Reached end of input.");
				break
			},
			Err(what) => {
				eprintln!("A read error occured: {:?}", what);
				break
			}
		}

		/* Parse and evaluate */
		if let Err(what) = c.process_line(&line){
			eprintln!("{}", what);
		} else {
			/* Drives the currently pending instructions to
			 * completion. */
			let mut saturate = || loop{
				let status = c.execute_next_instruction();

				use birl::vm::ExecutionStatus as Es;
				match status{
					Ok(Es::Quit) => break Ok(()),
					Err(what)    => break Err(what),
					_ => {}
				}
			};
			
			if let Err(what) = saturate(){
				eprintln!("{}", what);
			}
		}
	}

	/* Make sure the output is flushed */
	c.set_stdout(None).unwrap().flush()
		.expect("Could not flush io::stdout().");
}
