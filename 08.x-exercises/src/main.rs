mod vector;



fn main() {
	let data_set = vec![6, 4, 3, 9, 2, 7, 7, 1, 2, 7];
	// sorted:                    1, 2, 2, 3, 4, 6, 7, 7, 7, 9
	// mean: 4.8
	// median: 5 (4 & 6)
	// mode: 7

	assert_eq!(vector::mean(&data_set), 4.8);
	assert_eq!(vector::median(&data_set), 5.0);
	assert_eq!(vector::mode(&data_set), vec![7]);
}
