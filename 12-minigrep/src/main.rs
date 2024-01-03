use std::{env, process};

use minigrep::{Config, run};



fn main() {
	let config = Config::build(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});


	let err = run(config);

	if let Err(err) = err {
		eprintln!("Application error: {err}");
		process::exit(1);
	}
}
