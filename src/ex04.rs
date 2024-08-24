use crate::ex03::eval_formula;

fn is_valid_formula(formula: &str) -> bool {
	let valids: &str = "01!&|^>=";
	let formula_chars: Vec<char> = formula.chars().collect();

	for c in formula_chars {
		if !valids.contains(c) && !(c >= 'A' && c <= 'Z') {
			return false
		}
	}
	return true;
}

fn is_same_vector(v1: &Vec<bool>, v2: &Vec<bool>) -> bool {
	let mut i: usize = 0;
	let length: usize = v1.len();

	if length != v2.len() {
		return false;
	}

	while i < length {
		if v1[i] != v2[i] {
			return false;
		}
		i += 1;
	}

	return true;
}

fn exists_in(v: &Vec<bool>, collection: &Vec<Vec<bool>>) -> bool {
	let mut i: usize = 0;
	let length: usize = collection.len();

	while i < length {
		if is_same_vector(&collection[i], &v) {
			return true;
		}
		i += 1;
	}

	return false;
}

fn get_variables(formula: &str) -> Vec<char> {
	let formula_chars: Vec<char> = formula.chars().collect();
	let mut variables: Vec<char> = Vec::new();

	for c in formula_chars {
		if !variables.contains(&c) && (c >= 'A' && c <= 'Z') {
			variables.push(c);
		}
	}

	return variables;
}

fn generate_combinations(len: usize) -> Vec<Vec<bool>> {
	let mut combinations: Vec<Vec<bool>> = Vec::new();
	while combinations.len() < 2_usize.pow(len as u32) {
		// i is the current position of the switch we are trying to flip.
		// It will be decremented each time we know both true and false
		// have been tried
		let mut i: usize = len - 1;
		// We get the last pushed value or an all-false vector
		let mut to_push: Vec<bool> = combinations
								.last()
								.unwrap_or(&vec![false; len])
								.to_vec();
		loop {
			let mut opposite: Vec<bool> = to_push.clone();
			opposite[i] = !opposite[i];

			if exists_in(&to_push, &combinations)
			{
				if exists_in(&opposite, &combinations) {
					i -= 1;
				} else {
					to_push = opposite;
				}
			}
			else {
				break;
			}
		}
		combinations.push(to_push);
	}
	return combinations;
}

fn replace_values(formula: &str, combi: &Vec<bool>, variables: &Vec<char>) -> String {
	let mut replaced: String = formula.to_string();

	for (i, variable) in variables.into_iter().enumerate() {
		let c: char = if combi[i] { '1' } else { '0' };
		replaced = replaced.replace(variable.to_owned(), c.to_string().as_str());
	}

	return replaced;
}

pub fn print_truth_table(formula: &str) {
	if !is_valid_formula(formula) {
		return println!("Invalid formula");
	}

	let variables: Vec<char> = get_variables(formula);
	let combinations: Vec<Vec<bool>> = generate_combinations(variables.len());

	for variable in variables.iter() {
		print!("{}\t", variable);
	}
	print!("=\n\n");

	for combi in combinations.iter() {
		for value in combi {
			print!("{}\t", *value as u8);
		}
		let replaced: String = replace_values(formula, combi, &variables);
		println!("{}", eval_formula(replaced.as_str()));
	}
	print!("\n");
}
