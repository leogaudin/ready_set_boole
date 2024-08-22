use colored::Colorize;

mod ex00;
use ex00::adder;

mod ex01;
use ex01::multiplier;

use rand::RngCore;

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
		let mut rng = rand::thread_rng();
		let a: u32 = rng.next_u32() / std::u16::MAX as u32;
		let b: u32 = rng.next_u32() / std::u16::MAX as u32;
        println!(
			"{}\t{} * {} = {}",
			if multiplier(a, b) == a * b { "OK".green().bold() } else { "KO".red().bold() },
			a,
			b,
			multiplier(a, b),
		);
    }
}
