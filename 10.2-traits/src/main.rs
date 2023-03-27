mod vector;

use vector::ArithmeticOperations;



trait OtherTrait {

}



#[allow(unused)]
struct Vector<T> {
	x: T,
	y: T
}



fn main() {
	let data_set = vec![6, 4, 3, 9, 2, 7, 7, 1, 2, 7];
	// sorted:                    1, 2, 2, 3, 4, 6, 7, 7, 7, 9
	// mean: 4.8
	// median: 5 (4 & 6)
	// mode: 7

	assert_eq!(data_set.mean(), 4.8);
	assert_eq!(data_set.median(), 5.0);
	assert_eq!(data_set.mode(), vec![7]);
}



// argument type must implement ArithmeticOperations
fn _argument_must_impl(_item: &impl ArithmeticOperations) {
	// ...
}

// trait bound syntax (impl in generic)
fn _trait_bound<T: ArithmeticOperations>(_item: &T) {
	// ...
}



fn _impl_both<T: ArithmeticOperations + OtherTrait>(_item: &T) {
	// ...
}

fn _impl_both_where<T>(_item: &T)
where
	T: ArithmeticOperations + OtherTrait
{
	// ...
}



// only allows returning of one concrete type, not possibly multiple
fn _returns_arithmetic_impl() -> impl ArithmeticOperations {
	return vec![1];
}



// implement function for Vector, only if it's a Vector<T> where T implements both ArithmeticOperations and OtherTrait
impl<T: ArithmeticOperations + OtherTrait> Vector<T> {
	//
}



// implement ArithmeticOperations on any type implementing OtherTrait (blanket implmentations)
impl <T: OtherTrait> ArithmeticOperations for T {

}
