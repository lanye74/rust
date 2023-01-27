// sigh
// stack vs heap

// the stack is fast. it's a last-in first-out queue where every value thrown on it has a fixed size
// push values onto it, pop values off of it, it's the stack
// the heap is slower. it's what you would normally want: you request space on the stack, which gets allocated
// the heap allocater returns a pointer that gives you that space to access
// note that the heap pointer can be put on the stack

// function calls and their local variables are put on the stack
// you want to minimize data on the heap.



fn main() {
	// this is "immutable" according to the book? but this works
	// let mut my_string = "hello, world!";

	// my_string = "abcdsgdkjfdkjkjlfdslkjfdsskjlgsdkgfjdkgflklsgjflkgdsgsdgsdgfsdgsdgf";

	// println!("{}", my_string);

	// further research shows that string literals are "fixed length"... but i can assign my_string to much longer values...
	// whatever. use string literal for constants, and otherwise, use String ig

	// alternate: "Hello".to_string();
	let mut mut_string = String::from("Hello");

	mut_string.push(',');
	mut_string.push_str(" world!");

	// simple types, like ints, are cloned
	// this is because they have a fixed size and are on the stack, thus quick to clone
	let int1 = 5;
	let int2 = int1;

	// i'm not sure if there's a better way to format the address but
	assert_ne!(format!("{:p}", &int1), format!("{:p}", &int2));

	let str1 = String::from("hello");
	let _str2 = str1;

	// str2 is a reference to str1, but also, we are *moving* str1 into str2
	// you could consider this a shallow copy (creating another poiner to s1's data; reminder that s1 itself is a pointer)
	// except for the fact that str1 is being dereferenced
	// this happens because str1 would be dropped from memory twice if it wasn't

	// thus, because str1 has been moved, it's not valid in this expression below.
	// but if it was, it would assert as true
	// assert_eq!(format!("{:p}", &str1), format!("{:p}", &str2));

	// if you do want to deep copy:
	let str3 = String::from("hi");
	let str4 = str3.clone();

	assert_ne!(format!("{:p}", &str3), format!("{:p}", &str4));

}



// this function doesn't work because it prints the address of the argument (?) I'm not sure
// it would be lovely to have a function that prints the address of a given variable. alas

// fn format_address(variable: &i32) -> String {
// 	return format!("{:p}", &variable);
// }
