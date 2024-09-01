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

fn generate_sets(variables: Vec<char>, formula: &str) -> String {
	let mut result: String = String::new();
	let var_len: usize = variables.len();
	let mut set: Vec<bool> = vec![false; var_len];

	for binary_combination in 0..2usize.pow(var_len as u32) {
		let mut line: String = String::new();
		for position in 0..var_len {
			set[position] = binary_combination & (1 << position) != 0;
			line.push_str(&format!("{}\t", set[position] as u8));
		}
		let tree: TreeNodeRef = replace_variables(create_tree(formula), &set, &variables);
		let eval: bool = eval_tree(tree.clone());
		line.push_str(&eval.to_string());
		line.push_str("\n");
		result.push_str(&line);
	}

	return result;
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

	table.push_str(&generate_sets(variables, formula));
	return table;
}

pub fn print_truth_table(formula: &str) {
	println!("{}", generate_truth_table(formula));
}
