use crate::{ex04::get_variables, ex06::conjunctive_normal_form, tree::{create_tree, NodeValue, TreeNodeRef}};

fn get_union(sets: Vec<Vec<i32>>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for set in sets.iter() {
		for elem in set.iter() {
			if !result.contains(elem) {
				result.push(*elem);
			}
		}
	}

	return result;
}

fn get_intersection(set1: Vec<i32>, set2: Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for elem in set1.iter() {
		if set2.contains(elem) {
			result.push(*elem);
		}
	}

	return result;
}

fn get_complement(set: Vec<i32>, universe: Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for elem in universe.iter() {
		if !set.contains(elem) {
			result.push(*elem);
		}
	}

	return result;
}

fn get_set(variable: char, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let index: usize = (variable as u8 - 'A' as u8) as usize;
	return sets[index].clone();
}

fn eval_tree(node: TreeNodeRef, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Variable(variable)) => return get_set(variable, sets),
		Some(NodeValue::Operator('!')) => {
			let child: Vec<i32> = eval_tree(node.get_left().unwrap(), sets.clone());
			let universe: Vec<i32> = get_union(sets.clone());
			return get_complement(child, universe);
		}
		Some(NodeValue::Operator(operator)) => {
			let left: Vec<i32> = eval_tree(node.get_left().unwrap(), sets.clone());
			let right: Vec<i32> = eval_tree(node.get_right().unwrap(), sets.clone());
			match operator {
				'&' => return get_intersection(left, right),
				'|' => return get_union(vec![left, right]),
				_ => panic!("Invalid operator"),
			}
		}
		Some(NodeValue::Value(_)) => panic!("Value in expression"),
		None => panic!("Node has no value"),
	}
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let tree: TreeNodeRef = create_tree(conjunctive_normal_form(formula).as_str());

	let mut variables: Vec<char> = get_variables(formula);
	if variables.len() > sets.len() {
		panic!("Not enough sets");
	}
	variables.sort();

	let mut result: Vec<i32> = eval_tree(tree, sets);
	result.sort();

	return result;
}
