use crate::ex04::generate_truth_table;

pub fn sat(formula: &str) -> bool {
	let truth_table: String = generate_truth_table(formula);
	let lines = truth_table.lines();
	for line in lines {
		if line.ends_with("true") {
			return true;
		}
	}
	return false;
}
