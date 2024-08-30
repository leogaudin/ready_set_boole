// Double negation (A!! === A)
// Equivalence (AB= === AB>BA>&)
// Material condition (AB> === A!B|)
// De Morgan's laws (AB|! === A!B!&), (AB&! === A!B!|)
// Distributivity (ABC|& === AB&AC&|), (ABC&| === AB|AC|&)

use crate::tree::{create_tree, tree_to_rpn, NodeValue, TreeNode, TreeNodeRef};

pub fn remove_double_negation(node: TreeNodeRef) -> TreeNodeRef {
    let node = node.borrow();
    match node.get_val() {
        Some(NodeValue::Operator('!')) => {
            let left: TreeNodeRef = node.get_left().unwrap();
			let left = left.borrow();
			if let Some(NodeValue::Operator('!')) = left.get_val() {
				return remove_double_negation(left.get_left().unwrap());
			}
			return node.clone();
		}
		Some(NodeValue::Value(_)) | Some(NodeValue::Variable(_)) => return node.clone(),
		_ => {
			let new_node: TreeNodeRef = TreeNode::new_from(node.get_val().unwrap());
			if let Some(left) = node.get_left() {
				new_node.borrow_mut().set_left(remove_double_negation(left));
			}
			if let Some(right) = node.get_right() {
				new_node.borrow_mut().set_right(remove_double_negation(right));
			}
			return new_node;
		}
	}
}

pub fn remove_equivalence(node: TreeNodeRef) -> TreeNodeRef {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Operator('=')) => {
			let left: TreeNodeRef = node.get_left().unwrap();
			let right: TreeNodeRef = node.get_right().unwrap();
			let left = left.borrow();
			let right = right.borrow();
			let new_left: TreeNodeRef = TreeNode::new_with_children(
				NodeValue::Operator('>'),
				left.clone(),
				right.clone(),
			);
			let new_right: TreeNodeRef = TreeNode::new_with_children(
				NodeValue::Operator('>'),
				right.clone(),
				left.clone(),
			);

			return TreeNode::new_with_children(
				NodeValue::Operator('&'),
				remove_equivalence(new_left),
				remove_equivalence(new_right),
			);
		}
		Some(NodeValue::Value(_)) | Some(NodeValue::Variable(_)) => return node.clone(),
		_ => {
			let new_node: TreeNodeRef = TreeNode::new_from(node.get_val().unwrap());
			if let Some(left) = node.get_left() {
				new_node.borrow_mut().set_left(remove_equivalence(left));
			}
			if let Some(right) = node.get_right() {
				new_node.borrow_mut().set_right(remove_equivalence(right));
			}
			return new_node;
		}
	}
}

pub fn remove_de_morgan(node: TreeNodeRef) -> TreeNodeRef {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Operator('!')) => {
			let left: TreeNodeRef = node.get_left().unwrap();
			let left = left.borrow();
			match left.get_val() {
				Some(NodeValue::Operator('|')) => {
					let right: TreeNodeRef = left.get_right().unwrap();
					let left: TreeNodeRef = left.get_left().unwrap();
					let new_left: TreeNodeRef = TreeNode::new_from(NodeValue::Operator('!'));
					new_left.borrow_mut().set_left(left);
					let new_right: TreeNodeRef = TreeNode::new_from(NodeValue::Operator('!'));
					new_right.borrow_mut().set_left(right);

					return TreeNode::new_with_children(
						NodeValue::Operator('&'),
						remove_de_morgan(new_left),
						remove_de_morgan(new_right),
					);
				}
				Some(NodeValue::Operator('&')) => {
					let right: TreeNodeRef = left.get_right().unwrap();
					let left: TreeNodeRef = left.get_left().unwrap();
					let new_left: TreeNodeRef = TreeNode::new_from(NodeValue::Operator('!'));
					new_left.borrow_mut().set_left(left);
					let new_right: TreeNodeRef = TreeNode::new_from(NodeValue::Operator('!'));
					new_right.borrow_mut().set_left(right);
					return TreeNode::new_with_children(
						NodeValue::Operator('|'),
						remove_de_morgan(new_left),
						remove_de_morgan(new_right),
					);
				}
				_ => return node.clone(),
			}
		}
		Some(NodeValue::Value(_)) | Some(NodeValue::Variable(_)) => return node.clone(),
		_ => {
			let new_node: TreeNodeRef = TreeNode::new_from(node.get_val().unwrap());
			if let Some(left) = node.get_left() {
				new_node.borrow_mut().set_left(remove_de_morgan(left));
			}
			if let Some(right) = node.get_right() {
				new_node.borrow_mut().set_right(remove_de_morgan(right));
			}
			return new_node;
		}
	}
}

pub fn remove_material_condition(node: TreeNodeRef) -> TreeNodeRef {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Operator('>')) => {
			let left: TreeNodeRef = node.get_left().unwrap();
			let right: TreeNodeRef = node.get_right().unwrap();
			let new_left: TreeNodeRef = TreeNode::new_from(NodeValue::Operator('!'));
			new_left.borrow_mut().set_left(left);

			return TreeNode::new_with_children(
				NodeValue::Operator('|'),
				remove_material_condition(new_left),
				remove_material_condition(right),
			);
		}
		Some(NodeValue::Value(_)) | Some(NodeValue::Variable(_)) => return node.clone(),
		_ => {
			let new_node: TreeNodeRef = TreeNode::new_from(node.get_val().unwrap());
			if let Some(left) = node.get_left() {
				new_node.borrow_mut().set_left(remove_material_condition(left));
			}
			if let Some(right) = node.get_right() {
				new_node.borrow_mut().set_right(remove_material_condition(right));
			}
			return new_node;
		}
	}
}

pub fn tree_to_nnf(tree: TreeNodeRef) -> TreeNodeRef {
	let tree: TreeNodeRef = remove_equivalence(tree);
	let tree: TreeNodeRef = remove_material_condition(tree);
	let tree: TreeNodeRef = remove_de_morgan(tree);
	let tree: TreeNodeRef = remove_double_negation(tree);

	return tree;
}

pub fn negation_normal_form(formula: &str) -> String {
	let tree: TreeNodeRef = tree_to_nnf(create_tree(formula));
	return tree_to_rpn(tree).as_str().to_string();
}
