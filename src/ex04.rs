use crate::tree::{create_tree, eval_tree, replace_variables, TreeNodeRef};

fn is_valid_formula(formula: &str) -> bool {
	let valids: &str = "01!&|^>=";
	let formula_chars: Vec<char> = formula.chars().collect();

	for c in formula_chars {
		if !valids.contains(c) && !(c.is_ascii_uppercase()) {
			return false;
		}
	}
	return true;
}

pub fn get_variables(formula: &str) -> Vec<char> {
	let formula_chars: Vec<char> = formula.chars().collect();
	let mut variables: Vec<char> = Vec::new();

	for c in formula_chars {
		if !variables.contains(&c) && c.is_ascii_uppercase() {
			variables.push(c);
		}
	}

	variables.sort();

	return variables;
}

fn generate_sets(variables: Vec<char>, set: &mut Vec<bool>, index: usize, formula: &str) -> String {
	if index == variables.len() {
		let mut line: String = String::new();
		for value in set.iter() {
			line.push_str(&format!("{}\t", *value as u8));
		}
		let tree: TreeNodeRef = replace_variables(create_tree(formula), set, &variables);
		let eval: bool = eval_tree(tree.clone());
		line.push_str(&eval.to_string());
		line.push_str("\n");
		return line;
	}

	let mut line = String::new();
	set[index] = false;
	line.push_str(&generate_sets(variables.clone(), set, index + 1, formula));
	set[index] = true;
	line.push_str(&generate_sets(variables.clone(), set, index + 1, formula));
	return line;
}

pub fn generate_truth_table(formula: &str) -> String {
	if !is_valid_formula(formula) {
		return "Invalid formula".to_string();
	}

	let variables: Vec<char> = get_variables(formula);
	let mut table: String = String::new();
	for variable in &variables {
		table.push_str(&format!("{}\t", variable));
	}
	table.push_str("=\n");

	let mut set: Vec<bool> = vec![false; variables.len()];
	table.push_str(&generate_sets(variables, &mut set, 0, formula));
	return table;
}

pub fn print_truth_table(formula: &str) {
	println!("{}", generate_truth_table(formula));
}
