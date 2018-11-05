extern crate birl;

use std::env::args;
use birl::context::Context;

fn start_interactive_console() {
	/* Print heading info. */
	eprintln!("O SHELL QUE CONSTRÓI FIBRA. VERSÃO {}", birl::context::BIRL_VERSION);
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

    c.set_interactive_mode();

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

            use birl::vm::ExecutionStatus as Es;
            loop {
                match c.execute_next_instruction() {
                    Ok(Es::Quit) => {
                        eprintln!("Saindo...");
                        return;
                    }
                    Ok(Es::Halt) => break,
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
		}
	}

	/* Make sure the output is flushed */
	c.set_stdout(None).unwrap().flush()
		.expect("Could not flush io::stdout().");
}

fn print_help() {
	Context::print_version();

	println!("Ta querendo ajuda, cumpade?");
	println!("O uso é o seguinte: birl [opções] [arquivo ou arquivos]");
	println!("Cê pode passar mais de um arquivo, só que apenas um pode ter a seção \"SHOW\", que \
              é o ponto de partida do teu programa.");
	println!("As opções são as seguintes:");
	println!("\t-a ou --ajuda-o-maluco-ta-doente\t: Imprime essa mensagem de ajuda");
	println!("\t-v ou --versao\t\t\t\t: Imprime a versão do programa");
	println!("\t-s ou --string \"[codigo]\"\t\t: Executa o codigo na string ao inves de \
              um arquivo.");
	println!("\t-i ou --interativo\t\t\t\t: Inicia um console interativo pra rodar códigos");
}

/// Parameters passed through the command line
enum Param {
	PrintVersion,
	PrintHelp,
	/// Add a file to be processed
	InputFile(String),
	/// Processes code from a given string
	StringSource(String),
	/// Starts an interactive console for running code
	Interactive,
}

fn get_params() -> Vec<Param> {
	let mut arguments = args();
	let mut result: Vec<Param> = vec![];

	let _ = arguments.next().unwrap(); // Dispose of the first argument

	loop {
		if let Some(arg) = arguments.next() {
			match arg.as_str() {
				"-a" | "--ajuda-o-maluco-ta-doente" => result.push(Param::PrintHelp),
				"-v" | "--versao-cumpade" => result.push(Param::PrintVersion),
				"-i" | "--interativo" => result.push(Param::Interactive),
				"-s" | "--string" => {
					// The next argument is expected to be a string containing source code
					if let Some(code) = arguments.next() {
						result.push(Param::StringSource(code));
					} else {
						println!("Erro: O argumento {} precisa de um conteúdo logo em seguida, bixo.", arg);
					}
				}
				// Push the file to the result stack
				_ => result.push(Param::InputFile(arg))
			}
		} else {
			break;
		}
	}

	result
}

fn main() {
	let args = get_params();
	let mut interactive = false;

	let mut ctx = Context::new();

	if args.len() > 0 {
		for arg in args {
			match arg {
				Param::PrintHelp => print_help(),
				Param::Interactive => interactive = true,
				Param::PrintVersion => Context::print_version(),
				Param::InputFile(file) => {
					match ctx.add_file(file.as_str()) {
						Ok(_) => {}
						Err(e) => {
							println!("Ocorreu um erro ao adicionar o arquivo \"{}\" pro contexto : {}",
									 file.as_str(), e);
							// Exit? continue?
						}
					}
				}
				Param::StringSource(source) => {
					match ctx.add_source_string(source) {
						Ok(_) => {}
						Err(e) => {
							println!("Erro ao adicionar string de código ao contexto : {}", e);
						}
					}
				}
			}
		}
	} else {
		interactive = true;
	}

	if interactive {
		start_interactive_console();
	} else {
        /* Bind the Context interpreter to standard IO */
        let _ = ctx.set_stdin({
            use std::io;
            let reader = io::BufReader::new(io::stdin());
            Some(Box::new(reader))
        });
        let _ = ctx.set_stdout({
            use std::io;
            Some(Box::new(io::stdout()))
        });

		match ctx.start_program() {
			Ok(_) => {}
			Err(e) => println!("Erro de execução : {}", e),
		}
	}
}