use crate::ex03::eval_formula;

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

fn get_variables(formula: &str) -> Vec<char> {
	let formula_chars: Vec<char> = formula.chars().collect();
	let mut variables: Vec<char> = Vec::new();

	for c in formula_chars {
		if !variables.contains(&c) && c.is_ascii_uppercase() {
			variables.push(c);
		}
	}

	return variables;
}

fn replace_values(formula: &str, combi: &Vec<bool>, variables: &Vec<char>) -> String {
	let mut replaced: String = formula.to_string();

	for (i, variable) in variables.into_iter().enumerate() {
		let c: char = if combi[i] { '1' } else { '0' };
		replaced = replaced.replace(variable.clone(), c.to_string().as_str());
	}

	return replaced;
}

fn print_sets(variables: Vec<char>, set: &mut Vec<bool>, index: usize, formula: &str) {
	if index == variables.len() {
		for value in set.iter() {
			print!("{}\t", *value as u8);
		}
		let replaced: String = replace_values(formula, set, &variables);
		println!("{}", eval_formula(replaced.as_str()));
		return;
	}

	set[index] = false;
	print_sets(variables.clone(), set, index + 1, formula);
	set[index] = true;
	print_sets(variables.clone(), set, index + 1, formula);
}

pub fn print_truth_table(formula: &str) {
	if !is_valid_formula(formula) {
		println!("Invalid formula");
		return;
	}

	let variables: Vec<char> = get_variables(formula);
	for variable in &variables {
		print!("{}\t", variable);
	}
	print!("=\n\n");

	let mut initial_set: Vec<bool> = vec![false; variables.len()];
	print_sets(variables, &mut initial_set, 0, formula);
	print!("\n");
}
