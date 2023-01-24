use std::{io, cmp::Ordering};
use rand::Rng;



// :: vs .
// :: is for accessing STATIC members/properties of namespaces/classes
// . is for accessing INSTANCE members/properties of classes/objects
// e.g. the ::new() method is a STATIC property of string
// the .read_line() method is an INSTANCE property of the Stdin class



fn main() {
	let target_number = rand::thread_rng().gen_range(1..=100);

	println!("Guessing game!");
	println!("Guess the number between 1 and 100. Type quit to quit.");


	// shorthand for while(true)
	loop {
		println!("Input your guess: ");

		let mut guess = String::new();

		let stdin = io::stdin();

		stdin
			// takes the output address as a parameter, kinda like that one audio analyzer function
			// note: references are immutable by default; thus the mut is necessary
			// note again: this appends to the string's contents, not overwrites
			.read_line(&mut guess)
			// note again again: read_line returns a Result of Ok and Err;
			// if Err, the expect command... errors, if Ok, it just returns the value
			.expect("Failed to read line.");

		// overwriting a variable with another let is called shadowing
		// let guess: u32 = guess.trim().parse().expect("Invalid input!");

		// using a match here lets us not crash the program on invalid input
		// turbofish my beloved :)               ::<>
		let guess: u32 = match guess.trim().parse::<>() {
			Ok(num) => num,
			Err(_) => {
				if guess.trim().to_lowercase() == "quit" {
					break;
				}

				println!("Invalid guess!");
				continue;
			}
		};

		// i really wanted to use an if-else here but honestly this is a really clean pattern
		match guess.cmp(&target_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
