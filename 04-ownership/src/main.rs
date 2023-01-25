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

	// my_string = "abcd";

    // println!("{}", my_string);
}
