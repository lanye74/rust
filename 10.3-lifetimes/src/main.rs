fn main() {
    println!("Hello, world!");

	/*
	let outer; // lifetime A begins

	{
		let inner = 5; // lifetime B begins
		outer = &inner;
	} // lifetime B ends

	println!("{}", outer);

} // lifetime A ends

	outer refers to a lifetime that is shorter than itself; so code doesn't compile	*/

	let inner = 5; // lifetime B begins
	let outer = &inner; // lifetime A begins

	// outer refers to a lifetime that is longer than itself; no issue

	println!("{}", outer);

	// lifetime syntax
	// &i32        = ref
	// &'a i32     = ref w lifetime
	// &'a mut i32 = mut ref w lifetime

	// lifetime name: a
	// lifetime annotation: 'a
	// lifetime parameter: i32/mut i32
	// generally are very short like generic types (T)

	// lifetime annotations tell rust how lifetime parameters of multiple references relate to one another


	let bel = "bel'veth";

	{
		let kayn = "kayn";
		let longest = longest_slice(bel, kayn);

		// no issue
		println!("{}", longest);
	}

	// throws error. the value could be kayn, which has been dropped
	// println!("{}", longest);
}



// this function fails because rust doesn't know if the reference returned is slice_one or slice_two
// because of this, the borrow checker also doesn't know if the returned value will stay in lifetime (see above)
// fn longest_slice(slice_one: &str, slice_two: &str) -> &str {
// 	let slice_one_longer = slice_one.len() > slice_two.len();

// 	if slice_one_longer {
// 		return slice_one;
// 	} else {
// 		return slice_two;
// 	}
// }



// new lifetype annotation states that both slices will live at least as long as 'a
// this doesn't change the lifetime of slice_one and slice_two.
// rather any values that don't live as long as 'a should be rejected
// this forces the return value to be in the same scope as the most deeply nested paramter
fn longest_slice<'a>(slice_one: &'a str, slice_two: &'a str) -> &'a str {
	let slice_one_longer = slice_one.len() > slice_two.len();

	if slice_one_longer {
		return slice_one;
	} else {
		return slice_two;
	}
}
