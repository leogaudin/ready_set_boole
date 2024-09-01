pub fn reverse_map(n: f64) -> (u16, u16) {
	let n: u32 = (n * u32::MAX as f64) as u32;
	let x: u16 = (n >> 16) as u16;
	let y: u16 = ((n << 16) >> 16) as u16;

	return (x, y);
}
