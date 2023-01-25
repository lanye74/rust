fn main() {

	// if statements conditions must be bools
	// no funny type coercion like in js
	let value: bool = true;

	if value == true {
		println!("true");
	} else if value == false {
		println!("false");
	} else {
		println!("???");
	}


	// if else arms must be same type
	let _ternary = if value == true {5} else {84};


	// loops are expressions
	let mut i = 0;

	let n = loop {
		// unfortunately i can't do (++i == 10) because uhhhh yup ig
		// there's no increment/decrement operators and even if there were they return ()
		// o well
		i += 1;

		if i == 10 {
			break i * 2;
		}
	};


	// this is neat ig
	'named_loop: loop {
		if n == 20 {
			break 'named_loop;
		}
	}


}
