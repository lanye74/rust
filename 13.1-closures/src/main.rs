#[allow(unused_variables)]
fn main() {
	// closures capture their enivronment, like a callback
	// typing this explicitly is not necessary - both can be omitted
	let typed_closure = |num: u32| -> u32 {
		return num;
	};

	// closures do not seem to support generics
	// if you do not add types to the input - the compiler will infer it on first call,
	// then complain on any subsequent calls not of the same time

	// by default - closures immutably borrow their outside environment

	let mut outside_val = vec![1, 2, 3];

	let immutable_borrow = || println!("{:?}", outside_val);
	immutable_borrow();


	let mut mutable_borrow = || outside_val.push(4);
	mutable_borrow();
	println!("{:?}", outside_val); // 7

	// consumes any references
	let move_borrow = move || println!("{:?}", outside_val);
	move_borrow();

	// throws error
	// println!("{:?}", outside_val);
}
