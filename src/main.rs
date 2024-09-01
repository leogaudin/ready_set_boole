use colored::Colorize;
use rand::RngCore;
// use tree::{print_tree, tree_to_rpn, TreeNodeRef};
use ex00::adder;
use ex01::multiplier;
use ex02::gray_code;
use ex03::eval_formula;
use ex04::{generate_truth_table, print_truth_table};
use ex05::negation_normal_form;
use ex06::conjunctive_normal_form;
use ex07::sat;
use ex08::powerset;
use ex09::eval_set;
use ex10::map;
use ex11::reverse_map;

mod ex00;
mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod ex05;
mod ex06;
mod ex07;
mod ex08;
mod ex09;
mod ex10;
mod ex11;
mod tree;

fn main() {
	chrono("test_ex00_adder", test_ex00_adder);
	chrono("test_ex01_multiplier", test_ex01_multiplier);
	chrono("test_ex02_gray_code", test_ex02_gray_code);
	chrono("test_ex03_eval_formula", test_ex03_eval_formula);
	chrono("test_ex04_truth_table", test_ex04_truth_table);
	chrono("test_ex05_negation_normal_form", test_ex05_negation_normal_form);
	chrono("test_ex06_conjunctive_normal_form", test_ex06_conjunctive_normal_form);
	chrono("test_ex07_sat", test_ex07_sat);
	chrono("test_ex08_powerset", test_ex08_powerset);
	chrono("test_ex09_eval_set", test_ex09_eval_set);
	chrono("test_ex10_map", test_ex10_map);
	chrono("test_ex11_reverse_map", test_ex11_reverse_map);
}

fn chrono<F>(name: &str, f: F)
where
	F: Fn(),
{
	let start = std::time::Instant::now();
	f();
	let duration = start.elapsed();
	println!("\x1b[1m{} executed in {:?}\x1b[0m", name, duration);
}

fn test_ex00_adder() {
	println!("\n{}", "EX00 - ADDER".bold());
	for _ in 0..42 {
		let mut rng = rand::thread_rng();
		let a: u32 = rng.next_u32() / 2;
		let b: u32 = rng.next_u32() / 2;
		println!(
			"{}\t{} + {} = {}",
			if adder(a, b) == a + b {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			a,
			b,
			adder(a, b),
		);
	}
}

fn test_ex01_multiplier() {
	println!("\n{}", "EX01 - MULTIPLIER".bold());
	for _ in 0..42 {
		let mut generator: rand::prelude::ThreadRng = rand::thread_rng();
		let a: u32 = generator.next_u32() / std::u16::MAX as u32;
		let b: u32 = generator.next_u32() / std::u16::MAX as u32;
		println!(
			"{}\t{} * {} = {}",
			if multiplier(a, b) == a * b {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			a,
			b,
			multiplier(a, b),
		);
	}
}

fn test_ex02_gray_code() {
	println!("\n{}", "EX02 - GRAY CODE".bold());
	let pairs: Vec<(u32, u32)> = vec![
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
		(std::u32::MAX, 2147483648),
	];
	for pair in pairs {
		println!(
			"{}\t{} → {}",
			if gray_code(pair.0) == pair.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			pair.0,
			gray_code(pair.0),
		);
	}
}

fn test_ex03_eval_formula() {
	println!("\n{}", "EX03 - BOOLEAN EVALUATION".bold());
	let formulas: Vec<(&str, bool)> = vec![
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
			if eval_formula(formula.0) == formula.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			formula.0,
			eval_formula(formula.0)
		);
	}
}

fn test_ex04_truth_table() {
	println!("\n{}", "EX04 - TRUTH TABLE".bold());
	let formulas: Vec<&str> = vec![
		"AB01&|", "AB&C|", "ABZK||=", "1WAH1|&",
		// "ABCDEFGHIJKLMNOPQRSTUVWXYZ&|&|&|&|&|&",
	];

	for formula in formulas {
		println!("{}", formula.bold());
		print_truth_table(formula);
	}
}

fn test_ex05_negation_normal_form() {
	println!("\n{}", "EX05 - NEGATION NORMAL FORM".bold());
	let formulas: Vec<&str> = vec![
		// Subject
		"AB&!",   // → A!B!|
		"AB|!",   // → A!B!&
		"AB>",    // → A!B|
		"AB=",    // → AB&A!B!&|
		"AB|C&!", // → A!B!&C!|
		// Double negation
		"A!!B!!|C&!!",
		"A!!B!!C!!&&",
		"A!!B!!1|&",
		// Equivalence
		"AB=",
		"AD&BE&=",
		"AB&CD&=",
		// Material condition
		"AB>",
		"AB=",
		// De Morgan
		"AB|C&!",
		"AB&C|!",
		// Negation position
		"AB&!C|!",
		"AB&CD&!E|&F|!",
	];

	for formula in formulas {
		let nnf: String = negation_normal_form(formula);
		let allowed: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ01&|!".chars().collect();
		println!(
			"{}",
			if generate_truth_table(formula) != generate_truth_table(nnf.as_str()) {
				"TRUTH TABLES KO".red().bold()
			} else {
				"TRUTH TABLES OK".green().bold()
			},
		);
		println!(
			"{}",
			if !(nnf.as_str().chars().all(|c| allowed.contains(&c))) {
				"NORMALIZATION KO".red().bold()
			} else {
				"NORMALIZATION OK".green().bold()
			},
		);

		println!("Formula: {}", formula.bold());
		println!("RPN output: {}", nnf.as_str().bold());
		// print!("Tree: ");
		// print_tree(nnf);
		// println!();
		println!();
	}
}

fn test_ex06_conjunctive_normal_form() {
	println!("\n{}", "EX06 - CONJUNCTIVE NORMAL FORM".bold());
	let formulas: Vec<&str> = vec![
		// Subject
		"AB&!",    // → A!B!|
		"AB|!",    // → A!B!&
		"AB|C&",   // → AB|C&
		"AB|C|D|", // → ABCD|||
		"AB&C&D&", // → ABCD&&&
		"AB&!C!|", // → A!B!C!||
		"AB|!C!&", // → A!B!C!&&
		// Random
		"AB&C|", // → BC|AC|&
		"AB&CD&|",
		"AB&CD&|EF&|",
		"AB&CD&|EF&|GH&|",
		"AB&CD&|EF&|GH&|IJ&|",
		"AA!^BB!^^",
	];

	for formula in formulas {
		let cnf: String = conjunctive_normal_form(formula);
		println!(
			"{}",
			if generate_truth_table(formula) != generate_truth_table(cnf.as_str()) {
				"TRUTH TABLES KO".red().bold()
			} else {
				"TRUTH TABLES OK".green().bold()
			},
		);

		println!(
			"{}",
			if check_conjunctions(cnf.as_str()) {
				"CONJUNCTIONS OK".green().bold()
			} else {
				"CONJUNCTIONS KO".red().bold()
			},
		);

		println!("Formula: {}", formula.bold());
		println!("RPN output: {}", conjunctive_normal_form(formula).bold());
		println!();
	}
}

fn test_ex07_sat() {
	println!("\n{}", "EX07 - SAT".bold());
	let large: String = conjunctive_normal_form("AA!^BB!^^");
	let formulas: Vec<(&str, bool)> = vec![
		("AB|", true),
		("AB&", true),
		("AA!&", false),
		("AA^", false),
		("AA!&BB!&|", false),
		("AA!^BB!^^", false),
		(large.as_str(), false),
	];

	for formula in formulas {
		println!(
			"{}\t{} {}",
			if sat(formula.0) == formula.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			formula.0.bold(),
			if sat(formula.0) {
				"can be satisfied"
			} else {
				"cannot be satisfied"
			},
		);
	}
}

fn test_ex08_powerset() {
	println!("\n{}", "EX08 - POWERSET".bold());
	let sets: Vec<Vec<i32>> = vec![
		vec![1, 2, 3],
		vec![],
		// vec![1, 2, 3, 4],
		// vec![1, 2, 3, 4, 5],
		// vec![1, 2, 3, 4, 5, 6],
		// vec![1, 2, 3, 4, 5, 6, 7],
		// vec![1, 2, 3, 4, 5, 6, 7, 8],
		// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
		// vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
	];

	for set in sets {
		println!("Powerset of {:?}:", set);
		for subset in powerset(set.clone()) {
			println!("\t{:?}", subset);
		}
		println!();
	}
}

fn test_ex09_eval_set() {
	println!("\n{}", "EX09 - EVAL SET".bold());
	let sets: Vec<Vec<Vec<i32>>> = vec![
		vec![vec![0, 1, 2], vec![0, 3, 4]],
		vec![vec![0, 1, 2], vec![3, 4, 5]],
		vec![vec![0, 1, 2]],
		vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 3]],
		vec![vec![42, 43], vec![44, 45, 46]],
		vec![vec![42, 43], vec![44, 45, 46]],
	];
	let formulas: Vec<(&str, Vec<i32>)> = vec![
		("AB&", vec![0]),
		("AB|", vec![0, 1, 2, 3, 4, 5]),
		("A!", vec![]),
		("A!", vec![3, 4, 5, 6, 7]),
		("A!B!|", vec![42, 43, 44, 45, 46]),
		("A!B!&", vec![]),
	];

	for (i, formula) in formulas.iter().enumerate() {
		println!(
			"{}\t{}\t{:?} → {:?}",
			if eval_set(formula.0, sets[i].clone()) == formula.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			formula.0.bold(),
			sets[i],
			eval_set(formula.0, sets[i].clone()),
		);
	}
}

fn test_ex10_map() {
	println!("\n{}", "EX10 - CURVE".bold());
	let coordinates: Vec<((u16, u16), f64)> = vec![
		((0, 0), 0.0),
		((124, 5345), 0.0018933343239811561),
		((42141, 5543), 0.6430219206127855),
		((u16::MAX, u16::MAX), 1.0),
	];

	for coord in &coordinates {
		println!(
			"{}\t{:?} → {}",
			if map(coord.0 .0, coord.0 .1) == coord.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			coord.0,
			map(coord.0 .0, coord.0 .1)
		);
	}
}

fn test_ex11_reverse_map() {
	println!("\n{}", "EX11 - INVERSE FUNCTION".bold());
	let coordinates: Vec<((u16, u16), f64)> = vec![
		((0, 0), 0.0),
		((124, 5345), 0.0018933343239811561),
		((42141, 5543), 0.6430219206127855),
		((u16::MAX, u16::MAX), 1.0),
	];

	for coord in &coordinates {
		println!(
			"{}\t{} → {:?}",
			if reverse_map(coord.1) == (coord.0 .0, coord.0 .1) {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			coord.1,
			reverse_map(coord.1)
		);
	}
}

fn check_conjunctions(formula: &str) -> bool {
	let mut i: usize = 0; // First & character
	let chars: Vec<char> = formula.chars().collect();
	while i < chars.len() {
		if chars[i] == '&' {
			break;
		}
		i += 1;
	}
	let mut j: usize = i;
	while j < chars.len() {
		if chars[j] != '&' {
			break;
		}
		j += 1;
	}
	return j == chars.len();
}
