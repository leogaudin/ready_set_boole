use crate::ex00::adder;

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
