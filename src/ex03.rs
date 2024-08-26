pub fn eval_formula(formula: &str) -> bool {
	let chars: Vec<char> = formula.chars().collect();
	let mut stack: Vec<bool> = Vec::new();

	for c in chars {
		if c == '0' || c == '1' {
			stack.push(c != '0');
		}
		else {
			if c == '!' {
				let last: bool = stack.pop().unwrap();
				stack.push(!last);
				continue;
			}

			if stack.len() < 2 {
				println!("Invalid expression: not enough operands");
				return false;
			}

			let b: bool = stack.pop().unwrap();
			let a: bool = stack.pop().unwrap();

			match c {
				'&' => stack.push(a && b),
				'|' => stack.push(a || b),
				'^' => stack.push(a ^ b),
				'>' => stack.push(!a || b),
				'=' => stack.push(a == b),
				_ => {
					println!("Invalid operator");
					return false;
				}
			}
		}
	}

	return stack.pop().unwrap();
}
