pub fn gray_code(n: u32) -> u32 {
	let mut gray: u32 = 0;
	let mut length: u8 = 0;

	// Instead of initializing it in 32,
	// we calculate the bit length to
	// improve performance
	while n >> length != 0 {
		length += 1;
	}

	gray |= n >> length;

	while length > 0 {
		gray <<= 1;

		let left_bit: u32 = n >> (length);
		let right_bit: u32 = n >> (length - 1);

		gray |= left_bit ^ right_bit;

		length -= 1;
	}

	return gray;
}
