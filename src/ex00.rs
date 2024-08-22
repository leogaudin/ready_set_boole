pub fn adder(a: u32, b: u32) -> u32 {
	let mut result: u32 = a ^ b;
	let mut carry: u32 = a & b;

	while carry != 0 {
		let shifted_carry: u32 = carry << 1;
		carry = result & shifted_carry;
		result = result ^ shifted_carry;
	}

	return result;
}
