use colored::Colorize;
use rand::RngCore;

mod ex00;
use ex00::adder;

mod ex01;
use ex01::multiplier;

mod ex02;
use ex02::gray_code;

fn main() {
    println!("\n{}", "EX00 - ADDER".normal().bold());
    for _ in 0..42 {
		let mut rng = rand::thread_rng();
		let a: u32 = rng.next_u32() / 2;
		let b: u32 = rng.next_u32() / 2;
        println!(
			"{}\t{} + {} = {}",
			if adder(a, b) == a + b { "OK".green().bold() } else { "KO".red().bold() },
			a,
			b,
			adder(a, b),
		);
    }

    println!("\n{}", "EX01 - MULTIPLIER".normal().bold());
    for _ in 0..42 {
		let mut generator: rand::prelude::ThreadRng = rand::thread_rng();
		let a: u32 = generator.next_u32() / std::u16::MAX as u32;
		let b: u32 = generator.next_u32() / std::u16::MAX as u32;
        println!(
			"{}\t{} * {} = {}",
			if multiplier(a, b) == a * b { "OK".green().bold() } else { "KO".red().bold() },
			a,
			b,
			multiplier(a, b),
		);
    }

	println!("\n{}", "EX02 - GRAY CODE".normal().bold());
	let pairs = [
		(0, 0),
		(1, 1),
		(2, 3),
		(3, 2),
		(4, 6),
		(5, 7),
		(6, 5),
		(7, 4),
		(8, 12),
		(42, 63),
		(424242, 345515),
		(std::u32::MAX, 2147483648)
	];
	for pair in pairs {
		println!(
			"{}\t{} → {}",
			if gray_code(pair.0) == pair.1 { "OK".green().bold() } else { "KO".red().bold() },
			pair.0,
			gray_code(pair.0),
		);
	}
}
