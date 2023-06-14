use std::{env, process};

use minigrep::{Config, run};



fn main() {
	// the book wants me to annotate the type. i will use turbofish instead. no one can stop me.
	let args = env::args().collect::<Vec<String>>();

	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});


	let err = run(config);

	if let Err(err) = err {
		eprintln!("Application error: {err}");
		process::exit(1);
	}
}
