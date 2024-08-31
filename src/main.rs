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
use ex04::{generate_truth_table, print_truth_table};

mod tree;
// use tree::{print_tree, tree_to_rpn, TreeNodeRef};

mod ex05;
use ex05::negation_normal_form;

mod ex06;
use ex06::conjunctive_normal_form;

mod ex07;
use ex07::sat;

fn main() {
	println!("\n{}", "EX00 - ADDER".normal().bold());
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

	println!("\n{}", "EX01 - MULTIPLIER".normal().bold());
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
			if eval_formula(formula.0) == formula.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			formula.0,
			eval_formula(formula.0)
		);
	}

	println!("\n{}", "EX04 - TRUTH TABLE".normal().bold());
	let formulas = [
		"AB01&|", "AB&C|", "ABZK||=", "1WAH1|&",
		// "ABCDEFGHIJKLMNOPQRSTUVWXYZ&|&|&|&|&|&",
	];

	for formula in formulas {
		print_truth_table(formula);
	}

	println!("\n{}", "EX05 - NEGATION NORMAL FORM".normal().bold());
	let formulas = [
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

		println!("Formula: {}", formula.normal().bold());
		println!("RPN output: {}", nnf.as_str().normal().bold());
		// print!("Tree: ");
		// print_tree(nnf);
		// println!();
		println!();
	}

	println!("\n{}", "EX06 - CONJUNCTIVE NORMAL FORM".normal().bold());
	let formulas = [
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

		println!("Formula: {}", formula.normal().bold());
		println!(
			"RPN output: {}",
			conjunctive_normal_form(formula).normal().bold()
		);
		println!();
	}
	println!("\n{}", "EX07 - SAT".normal().bold());
	let formulas = [
		("AB|", true),
		("AB&", true),
		("AA!&", false),
		("AA^", false),
		("AA!&BB!&|", false),
		("AA!^BB!^^", false),
	];

	for formula in formulas {
		println!(
			"{}\t{} {}",
			if sat(formula.0) == formula.1 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			formula.0,
			if sat(formula.0) {
				"can be satisfied"
			} else {
				"cannot be satisfied"
			},
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
