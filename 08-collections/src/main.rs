fn main() {
    println!("Hello, world!");

	let mut vector1: Vec<i32> = Vec::new();

	vector1.push(1);
	vector1.push(2);
	vector1.push(3);

	let vector2 = vec![4, 5, 6];

	// panics if element does not exist (me fr)
	let _el1 = &vector1[2];

	vector1.push(7);

	// errors because el1 is an immutable borrow
	// println!("the first element is {_el1}");

	// unwraps to None if does not exist
	let _el2 = vector2.get(2);

	for i in &vector2 {
		println!("element in vector 2: {i}");
	}

	for i in &mut vector1 {
		// iterate and mutate
		*i *= 2;
	}

	// also, while vectors can only hold single types, they can hold enum variants
}
