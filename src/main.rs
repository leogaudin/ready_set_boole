use colored::Colorize;
use rand::RngCore;

mod ex00;
use ex00::adder;

mod ex01;
use ex01::multiplier;

mod ex02;
use ex02::gray_code;

mod ex03;
use ex03::eval_formula;

mod ex04;
use ex04::print_truth_table;

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

	println!("\n{}", "EX03 - BOOLEAN EVALUATION".normal().bold());
	let formulas = [
		("10&", false),
		("10|", true),
		("11>", true),
		("10=", false),
		("1011||=", true),
		("1!0=", true),
		("10&!", true),
		("00&1|", true),
		("10&0|", false),
		("11&0|", true),
	];
	for formula in formulas {
		println!(
			"{}\t{}\t→\t{}",
			if eval_formula(formula.0) == formula.1 { "OK".green().bold() } else { "KO".red().bold() },
			formula.0,
			eval_formula(formula.0)
		);
	}

	println!("\n{}", "EX04 - TRUTH TABLE".normal().bold());
	let formulas = [
		"AB01&|",
		"AB&C|",
		"ABZK||=",
		"1WAH1|&",
		// "ABCDEFGHIJKLMN&|&|&|&|&|&",
	];

	for formula in formulas {
		print_truth_table(formula);
	}
}
