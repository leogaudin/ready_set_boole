pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = vec![];
	let len: usize = set.len();
	let powerset_len: usize = 2usize.pow(len as u32);

	for binary_combination in 0..powerset_len {
		let mut subset: Vec<i32> = vec![];

		for position in 0..len { // Check if the j-th bit is set in the binary combination
			if binary_combination & (1 << position) != 0 {
				subset.push(set[position]);
			}
		}

		result.push(subset);
	}

	return result;
}
