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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
		let pairs = [
			(32, 1241),
			(16, 2142),
			(42, 24412)
		];
		for pair in pairs {
			assert_eq!(
				adder(pair.0, pair.1),
				pair.0 + pair.1
			)
		}
	}
}


