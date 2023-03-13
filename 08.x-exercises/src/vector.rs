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

	let half_length = length / 2.0;

	// even number of elements = 2 median points
	if length % 2.0 == 0.0 {
		let left_median = (half_length - 1.0) as usize;
		let right_median = half_length as usize;

		medians.push(*sorted.get(left_median).unwrap());
		medians.push(*sorted.get(right_median).unwrap());
	} else {
		let median = (half_length - 0.5) as usize;

		medians.push(*sorted.get(median).unwrap());
	}


	let sum_medians = sum_vec(&medians) as f32;

	return sum_medians / medians.len() as f32;
}



pub fn mode(input: &Vec<i32>) -> Vec<i32> {
	let mut map: HashMap<i32, i32> = HashMap::new();
	let mut output: Vec<i32> = Vec::new();

	let mut highest_count = 1;

	for number in input.iter() {
		let times_appeared = map.entry(*number).or_insert(0);
		*times_appeared += 1;

		if *times_appeared == highest_count {
			output.push(*number);
		}

		if *times_appeared > highest_count {
			// reset array
			output = vec![*number];

			highest_count = *times_appeared;
		}
	}

	output.sort();

	return output;
}



fn sum_vec(input: &Vec<i32>) -> i32 {
	// input.iter().sum() is kinda cheating tbh
	let mut sum = 0;

	for number in input.iter() {
		sum += *number;
	}

	return sum;
}
