mod tree;
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

use colored::Colorize;
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

fn main() {
	test_ex00_adder();
	test_ex01_multiplier();
	test_ex02_gray_code();
	test_ex03_eval_formula();
	test_ex04_truth_table();
	test_ex05_negation_normal_form();
	test_ex06_conjunctive_normal_form();
	test_ex07_sat();
	test_ex08_powerset();
	test_ex09_eval_set();
	test_ex10_map();
	test_ex11_reverse_map();
}

fn test_ex00_adder() {
	println!("\n{}", "EX00 - ADDER".bold());
	let pairs: Vec<(u32, u32, u32)> = vec![
		(0, 0, 0),
		(1, 0, 1),
		(0, 1, 1),
		(1, 1, 2),
		(1, 2, 3),
		(2, 2, 4),
		(42, 42, 84),
	];
	for pair in pairs {
		println!(
			"{}\t{} + {} = {}",
			if adder(pair.0, pair.1) == pair.2 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			pair.0,
			pair.1,
			adder(pair.0, pair.1),
		);
	}
}

fn test_ex01_multiplier() {
	println!("\n{}", "EX01 - MULTIPLIER".bold());
	let pairs: Vec<(u32, u32, u32)> = vec![
		(0, 0, 0),
		(1, 0, 0),
		(0, 1, 0),
		(1, 1, 1),
		(1, 2, 2),
		(2, 2, 4),
		(42, 42, 1764),
	];

	for pair in pairs {
		println!(
			"{}\t{} * {} = {}",
			if multiplier(pair.0, pair.1) == pair.2 {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			pair.0,
			pair.1,
			multiplier(pair.0, pair.1),
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
		("0!", true),
		("1!", false),
		("00|", false),
		("10|", true),
		("01|", true),
		("11|", true),
		("10&", false),
		("11&", true),
		("11^", false),
		("10^", true),
		("00>", true),
		("01>", true),
		("10>", false),
		("11>", true),
		("00=", true),
		("11=", true),
		("10=", false),
		("01=", false),
		("11&0|", true),
		("10&1|", true),
		("11&1|", true),
		("11&1|1^", false),
		("01&1|1=", true),
		("01&1&1&", false),
		("0111&&&", false),
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
		"A",
		"A!",
		"AB|",
		"AB&",
		"AB^",
		"AB>",
		"AB=",
		"AA=",
		"ABC==",
		"AB>C>",
		"AB>A>A>",
	];

	for formula in formulas {
		println!("{}", formula.bold());
		print_truth_table(formula);
	}
}

fn test_ex05_negation_normal_form() {
	println!("\n{}", "EX05 - NEGATION NORMAL FORM".bold());
	let formulas: Vec<&str> = vec![
		"A",
		"A!",
		"AB&!",
		"AB|!",
		"AB>!",
		"AB=!",
		"ABC||",
		"ABC||!",
		"ABC|&",
		"ABC&|",
		"ABC&|!",
		"ABC^^",
		"ABC>>",
	];

	for formula in formulas {
		let nnf: String = negation_normal_form(formula);
		let allowed: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ01&|!".chars().collect();
		print!(
			"{}\t",
			if generate_truth_table(formula) != generate_truth_table(nnf.as_str()) {
				"TT KO".red().bold()
			} else {
				"TT OK".green().bold()
			},
		);
		print!(
			"{}\t\t",
			if !(nnf.as_str().chars().all(|c| allowed.contains(&c))) {
				"NNF KO".red().bold()
			} else {
				"NNF OK".green().bold()
			},
		);

		println!("{}\t→\t{}", formula.bold(), nnf.bold());
	}
}

fn test_ex06_conjunctive_normal_form() {
	println!("\n{}", "EX06 - CONJUNCTIVE NORMAL FORM".bold());
	let formulas: Vec<&str> = vec![
		"A",
		"A!",
		"AB&!",
		"AB|!",
		"AB>!",
		"AB=!",
		"ABC||",
		"ABC||!",
		"ABC|&",
		"ABC&|",
		"ABC&|!",
		"ABC^^",
		"ABC>>",
	];

	for formula in formulas {
		let cnf: String = conjunctive_normal_form(formula);
		print!(
			"{}\t",
			if generate_truth_table(formula) != generate_truth_table(cnf.as_str()) {
				"TT KO".red().bold()
			} else {
				"TT OK".green().bold()
			},
		);

		print!(
			"{}\t\t",
			if check_conjunctions(cnf.as_str()) {
				"CNF OK".green().bold()
			} else {
				"CNF KO".red().bold()
			},
		);

		println!("{}\t→\t{}", formula.bold(), conjunctive_normal_form(formula).bold());
	}
}

fn test_ex07_sat() {
	println!("\n{}", "EX07 - SAT".bold());

	let formulas: Vec<(&str, bool)> = vec![
		("A", true),
		("A!", true),
		("AA|", true),
		("AA&", true),
		("AA!&", false),
		("AA^", false),
		("AB^", true),
		("AB=", true),
		("AA>", true),
		("AA!>", true),
		("ABC||", true),
		("AB&A!B!&&", false),
		("ABCDE&&&&", true),
		("AAA^^", true),
		("ABCDE^^^^", true),
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
		vec![],
		vec![0],
		vec![0, 1],
		vec![0, 1, 2],
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
		vec![vec![]],
		vec![vec![]],
		vec![vec![42]],
		vec![vec![42]],
		vec![vec![1, 2, 3], vec![2, 3, 4]],
		vec![vec![0, 1, 2], vec![]],
		vec![vec![0, 1, 2], vec![]],
		vec![vec![0, 1, 2], vec![0]],
		vec![vec![0, 1, 2], vec![42]],
		vec![vec![0, 1, 2], vec![0]],
		vec![vec![0], vec![0, 1, 2]],
		vec![vec![0], vec![1, 2]],
		vec![vec![], vec![], vec![]],
		vec![vec![0], vec![1], vec![2]],
		vec![vec![0], vec![0], vec![0]],
		vec![vec![0], vec![0], vec![]],
		vec![vec![0], vec![0], vec![0]],
		vec![vec![0], vec![0], vec![0]],
		vec![vec![0], vec![0], vec![0]],
		vec![vec![0], vec![0], vec![0]],
	];

	let formulas: Vec<(&str, Vec<i32>)> = vec![
		("A", vec![]),
		("A!", vec![]),
		("A", vec![42]),
		("A!", vec![]),
		("A!B&", vec![4]),
		("AB|", vec![0, 1, 2]),
		("AB&", vec![]),
		("AB&", vec![0]),
		("AB&", vec![]),
		("AB^", vec![1, 2]),
		("AB>", vec![0, 1, 2]),
		("AB>", vec![1, 2]),
		("ABC||", vec![]),
		("ABC||", vec![0, 1, 2]),
		("ABC||", vec![0]),
		("ABC&&", vec![]),
		("ABC&&", vec![0]),
		("ABC^^", vec![0]),
		("ABC>>", vec![0]),
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
	let coordinates: Vec<(u16, u16)> = vec![
		(0, 0),
		(124, 5345),
		(42141, 5543),
		(u16::MAX, u16::MAX)
	];

	for coord in &coordinates {
		println!(
			"{}\t{:?} == {:?}",
			if reverse_map(map(coord.0, coord.1)) == (coord.0, coord.1) {
				"OK".green().bold()
			} else {
				"KO".red().bold()
			},
			reverse_map(map(coord.0, coord.1)),
			(coord.0, coord.1)
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
