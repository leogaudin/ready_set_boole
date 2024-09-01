pub fn map(x: u16, y: u16) -> f64 {
	let concatenated: u32 = (x as u32) << 16 | y as u32;
	return concatenated as f64 / u32::MAX as f64;
}
