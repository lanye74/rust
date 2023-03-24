fn main() {
	let list = vec![1, 2, 3];
	let _three = largest(&list);
}



fn largest<T>(list: &Vec<T>) -> &T {
	let /*mut*/largest = &list[0];

	for _item in list {
		// cannot compare because T may not implement the comparison operator trait
		// if item > largest {
		// 	largest = item;
		// }
	}

	return largest;
}



#[allow(unused)]
struct Vector<T> {
	x: T,
	y: T
}



impl<T> Vector<T> {
	#[allow(non_snake_case)]
	fn _midpoint_with_T(&self, other: Vector<T>) -> T {
		// math here; refer to below
		return other.x;
	}
}



impl Vector<f32> {
	fn _midpoint_with(&self, other: &Vector<f32>) -> Vector<f32> {
		let x_avg = (self.x + other.x) / 2.0;
		let y_avg = (self.y + other.y) / 2.0;

		return Vector {
			x: x_avg,
			y: y_avg
		};
	}
}



// something something constraints --> something something lifetimes
// i'll learn how to do this in 10.3
// fn average<T>(input: Vec<T>) -> T {
// 	let sum = input.iter().sum();
// 	convert to T?
// 	let length = input.len()/* as T*/;s

// 	return sum;
// }
