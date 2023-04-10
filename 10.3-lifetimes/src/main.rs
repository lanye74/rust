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

// outer refers to a lifetime that is shorter than itself; so code doesn't compile

*/


	let inner = 5;
	let outer = &inner;

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


	let bel = String::from("bel'veth");
	let longest;

	{
		let kayn = String::from("kayn");
		longest = longest_slice(&bel[..], &kayn[..]);

		// no issue
		println!("{}", longest);
	}

	// throws error. the value could be kayn, which has been dropped
	// println!("{}", longest);
}



// this function needs lifetimes because the function could return either slice_one or slice_two
// note the function could NOT return *just a slice* because then it would be dangling.
// because of this, the borrow checker also doesn't know if the returned value will stay in lifetime

// new lifetime annotation states that both slices will live at least as long as 'a
// this doesn't change the lifetime of slice_one and slice_two.
// rather any values that don't live as long as 'a should be rejected
// this forces uses of the return value to be in the same scope as the most deeply nested paramter
fn longest_slice<'a>(slice_one: &'a str, slice_two: &'a str) -> &'a str {
	let slice_one_longer = slice_one.len() > slice_two.len();

	if slice_one_longer {
		return slice_one;
	} else {
		return slice_two;
	}
}



// theoretically you would need a lifetime here... and you used to. however, you would be writing
// fn elision<'a>(input: &'a str) -> &'a str {
// which is a really common pattern which ends up being written constantly. so rust can infer some lifetimes
// on the ones it cannot infer, it throws an error and asks you to instead
fn elision(argument: &str) -> &str {
	return &argument[0..2];
}

// three rules to elide lifetimes. if the compiler cannot figure out all lifetimes in the signature,
// then lifetimes need to be manually specified.
// rule 1: each input (reference) parameter gets its own lifetime
// rule 2: if there is only one input reference parameter, that lifetime is assigned to the output parameter
// rule 3: if in a method, the lifetime of self is assigned to all output lifetime parameters





// structs can also take lifetimes
// the struct cannot outlive its reference
#[allow(unused)]
struct ContainsReference<'a> {
	reference: &'a str
}
