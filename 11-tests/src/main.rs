fn main() {
	println!("Hello, world!");
}



// code should only be compiled when running tests
#[cfg(test)]
mod tests {
	// expose all outside functions to tests
	// use super::*;

	// function is a test that should be run
	#[test]
	fn check_2_2_4() {
		// assert!(a) - success if a == true
		// assert!(cond) - success if cond evals to true (e.g. 2 + 2 == 4)
		// assert_eq!(a, b) - success if a == b
		// assert_ne!(a, b) - success if a != b


		println!("this text will not show");

		assert!(2 + 2 == 4);
	}

	#[test]
	fn panic_fails() {
		println!("this text will show");

		panic!(":(");
	}

	#[test]
	// if code does not panic, the test runner will say "test did not panic as expected"
	// optionally, #[should_panic(expected = "xyz should've happened")]
	#[should_panic]
	fn panic_passes() {
		panic!(":)");
	}

	#[test]
	fn result() -> Result<(), ()> {
		// this also allows you to use the ? operator
		return Ok(());
	}

	#[test]
	#[ignore]
	fn will_not_run_unless_specified() {
		panic!("gooby");
	}
}
