use crate::ex00::adder;

// pub fn multiplier(a: u32, b: u32) -> u32 {
// 	let mut result = 0;
// 	for _ in 0..b {
// 		result = adder(result, a);
// 	}
// 	return result;
// }

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
	let mut result: u32 = 0;

	while b != 0 {
		if b & 1 != 0 {
			result = adder(result, a);
		}
		a <<= 1;
		b >>= 1;
	}
	return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
		let pairs = [
			(32, 1241),
			(16, 2142),
			(42, std::u32::MAX / 42),
		];
		for pair in pairs {
			assert_eq!(
				multiplier(pair.0, pair.1),
				pair.0 * pair.1
			)
		}
	}
}
