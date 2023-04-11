#[allow(unused_imports)]
use std::iter::Sum;



fn main() {
	let list = vec![1, 2, 3];
	let _three = largest(&list);
}



fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
	let mut largest = &list[0];

	for item in list {
		if item > largest {
			largest = item;
		}
	}

	return largest;
}



#[allow(dead_code)]
struct Vector<T> {
	x: T,
	y: T
}



impl<T> Vector<T> {
	fn _midpoint_with_generic(&self, other: Vector<T>) -> T {
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



// fn average<'a, T>(input: &'a Vec<T>) -> f32
// where
// 	T: Sum + Sum<&'a T>,
// 	i32: Sum<&'a T>,
// 	f32: Sum<&'a T>
// {
// 	let sum = input.iter().sum::<f32>();

// 	let length = input.len() as f32;

// 	return sum / length;
// }



// something something constraints --> something something lifetimes
// i'll learn how to do this in 10.3
// fn average<T>(input: Vec<T>) -> T {
// 	let sum = input.iter().sum();
// 	convert to T?
// 	let length = input.len()/* as T*/;s

// 	return sum;
// }
