use crate::{ex05::tree_to_nnf, tree::{create_tree, tree_to_rpn, NodeValue, TreeNode, TreeNodeRef}};

// Distributivity (ABC|& === AB&AC&|), (ABC&| === AB|AC|&)
fn apply_cnf(node: TreeNodeRef) -> TreeNodeRef {
	let node: TreeNodeRef = tree_to_nnf(node);
	let node = node.borrow();
	match node.get_val() {
		// (A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
		Some(NodeValue::Operator('|')) => {
			let left: TreeNodeRef = apply_cnf(node.get_left().unwrap());
			let right: TreeNodeRef = apply_cnf(node.get_right().unwrap());
			let left = left.borrow();
			let right = right.borrow();
			match (left.get_val(), right.get_val()) {
				(Some(NodeValue::Operator('&')), _) => {
					let left_left: TreeNodeRef = left.get_left().unwrap();
					let left_right: TreeNodeRef = left.get_right().unwrap();
					let left_left = left_left.borrow();
					let left_right = left_right.borrow();
					let new_left: TreeNodeRef = TreeNode::new_with_children(
						NodeValue::Operator('|'),
						left_left.clone(),
						right.clone(),
					);
					let new_right: TreeNodeRef = TreeNode::new_with_children(
						NodeValue::Operator('|'),
						left_right.clone(),
						right.clone(),
					);
					return TreeNode::new_with_children(
						NodeValue::Operator('&'),
						apply_cnf(new_left),
						apply_cnf(new_right),
					);
				}
				(_, Some(NodeValue::Operator('&'))) => {
					let right_left: TreeNodeRef = right.get_left().unwrap();
					let right_right: TreeNodeRef = right.get_right().unwrap();
					let right_left = right_left.borrow();
					let right_right = right_right.borrow();
					let new_left: TreeNodeRef = TreeNode::new_with_children(
						NodeValue::Operator('|'),
						left.clone(),
						right_left.clone(),
					);
					let new_right: TreeNodeRef = TreeNode::new_with_children(
						NodeValue::Operator('|'),
						left.clone(),
						right_right.clone(),
					);
					return TreeNode::new_with_children(
						NodeValue::Operator('&'),
						apply_cnf(new_left),
						apply_cnf(new_right),
					);
				}
				_ => return node.clone(),
			}
		}
		Some(NodeValue::Value(_)) | Some(NodeValue::Variable(_)) => return node.clone(),
		_ => {
			let new_node: TreeNodeRef = TreeNode::new_from(node.get_val().unwrap());
			if let Some(left) = node.get_left() {
				new_node.borrow_mut().set_left(apply_cnf(left));
			}
			if let Some(right) = node.get_right() {
				new_node.borrow_mut().set_right(apply_cnf(right));
			}
			return new_node;
		}
	}
}

fn move_conjunctions_to_end(formula: &str) -> String {
	let mut result: String = String::new();
	let mut conjunctions: String = String::new();
	let mut i: usize = 0;
	let chars: Vec<char> = formula.chars().collect();
	while i < chars.len() {
		match chars[i] {
			'&' => conjunctions.push(chars[i]),
			_ => result.push(chars[i]),
		}
		i += 1;
	}
	result.push_str(conjunctions.as_str());
	return result;
}

pub fn conjunctive_normal_form(formula: &str) -> String {

	// Implement conjunctive normal form from negation normal form
	// Return the conjunctive normal form as a string
	let tree: TreeNodeRef = create_tree(formula);
	let cnf: TreeNodeRef = apply_cnf(tree);
	// let moved: String = tree_to_rpn(cnf).as_str().to_string();
	let moved: String = move_conjunctions_to_end(tree_to_rpn(cnf).as_str());
	// print_tree(cnf);

	return moved;
}
