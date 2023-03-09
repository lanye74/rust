use std::collections::HashMap;



pub fn mean(input: &Vec<i32>) -> f32 {
	let sum = sum_vec(input);

	let length = input.len() as f32;


	return sum as f32 / length;
}



pub fn median(input: &Vec<i32>) -> f32 {
	let length = input.len() as f32;

	let mut sorted = input.clone();
	sorted.sort();


	let mut medians: Vec<i32> = vec![];

	if length % 2.0 == 0.0 {
		// this is awful code
		medians.push(*sorted.get((length / 2.0 - 1.0) as usize).unwrap());
		medians.push(*sorted.get((length / 2.0) as usize).unwrap());
	} else {
		medians.push(*sorted.get((length / 2.0 - 0.5) as usize).unwrap());
	}


	let sum_medians = sum_vec(&medians) as f32;

	return sum_medians / medians.len() as f32;
}



pub fn mode(input: &Vec<i32>) -> Vec<i32> {
	let mut map: HashMap<i32, i32> = HashMap::new();

	for number in input.iter() {
		let count = map.entry(*number).or_insert(0);
		*count += 1;
	}

	// ?????????

	return vec![3];
}



fn sum_vec(input: &Vec<i32>) -> i32 {
	// input.iter().sum() is kinda cheating tbh
	let mut sum = 0;

	for number in input.iter() {
		sum += *number;
	}

	return sum;
}
