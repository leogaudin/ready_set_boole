use std::{cell::RefCell, cmp::Ordering, rc::Rc};

#[derive(Debug, Clone)]
pub enum NodeValue {
	Value(bool),
	Variable(char),
	Operator(char),
}

pub struct TreeNode {
	value: Option<NodeValue>,
	left: Option<TreeNodeRef>,
	right: Option<TreeNodeRef>,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
	pub fn new() -> TreeNodeRef {
		Rc::new(RefCell::new(TreeNode {
			value: None,
			left: None,
			right: None,
		}))
	}

	pub fn new_from(value: NodeValue) -> TreeNodeRef {
		Rc::new(RefCell::new(TreeNode {
			value: Some(value),
			left: None,
			right: None,
		}))
	}

	pub fn new_with_children(value: NodeValue, left: TreeNodeRef, right: TreeNodeRef) -> TreeNodeRef {
		Rc::new(RefCell::new(TreeNode {
			value: Some(value),
			left: Some(left),
			right: Some(right),
		}))
	}

	pub fn set_val(&mut self, value: NodeValue) {
		self.value = Some(value);
	}

	pub fn set_left(&mut self, left: TreeNodeRef) {
		self.left = Some(left);
	}

	pub fn set_right(&mut self, right: TreeNodeRef) {
		self.right = Some(right);
	}

	pub fn get_val(&self) -> Option<NodeValue> {
		self.value.clone()
	}

	pub fn get_left(&self) -> Option<TreeNodeRef> {
		self.left.clone()
	}

	pub fn get_right(&self) -> Option<TreeNodeRef> {
		self.right.clone()
	}

	pub fn clone(&self) -> TreeNodeRef {
		Rc::new(RefCell::new(TreeNode {
			value: self.value.clone(),
			left: self.left.clone(),
			right: self.right.clone(),
		}))
	}
}

// pub fn print_tree(node: TreeNodeRef) {
// 	let node = node.borrow();
// 	match node.get_val() {
// 		Some(NodeValue::Value(value)) => print!("{}", value),
// 		Some(NodeValue::Variable(variable)) => print!("{}", variable),
// 		Some(NodeValue::Operator('!')) => {
// 			print!("!(");
// 			print_tree(node.get_left().unwrap());
// 			print!(")");
// 		}
// 		Some(NodeValue::Operator(operator)) => {
// 			print!("(");
// 			print_tree(node.get_left().unwrap());
// 			print!("{}", operator);
// 			print_tree(node.get_right().unwrap());
// 			print!(")");
// 		}
// 		None => panic!("Node has no value"),
// 	}
// }

pub fn create_tree(expression: &str) -> TreeNodeRef {
	let mut stack: Vec<TreeNodeRef> = Vec::new();
	for c in expression.chars() {
		if c == ' ' {
			continue;
		}
		let node: TreeNodeRef = TreeNode::new();
		match c {
			'0' | '1' => {
				node.borrow_mut().set_val(NodeValue::Value(c == '1'));
			},
			'A'..='Z' => {
				node.borrow_mut().set_val(NodeValue::Variable(c));
			},
			'!' => {
				let left: TreeNodeRef = stack.pop().unwrap();
				node.borrow_mut().set_val(NodeValue::Operator(c));
				node.borrow_mut().set_left(left);
			}
			'&' | '|' | '^' | '>' | '=' => {
				let right: TreeNodeRef = stack.pop().unwrap();
				let left: TreeNodeRef = stack.pop().unwrap();
				node.borrow_mut().set_val(NodeValue::Operator(c));
				node.borrow_mut().set_left(left);
				node.borrow_mut().set_right(right);
			}
			_ => panic!("Invalid character in expression"),
		}
		stack.push(node);
	}

	match stack.len().cmp(&1usize) {
		Ordering::Greater => panic!("Too many operands, not enough operators."),
		Ordering::Equal => return stack.pop().unwrap(),
		Ordering::Less => panic!("Empty formula"),
	}
}

pub fn eval_tree(node: TreeNodeRef) -> bool {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Value(value)) => value,
		Some(NodeValue::Operator(operator)) => {
			if operator == '!' {
				return !eval_tree(node.get_left().unwrap());
			}
			let left: bool = eval_tree(node.get_left().unwrap());
			let right: bool = eval_tree(node.get_right().unwrap());
			match operator {
				'&' => return left && right,
				'|' => return left || right,
				'^' => return left ^ right,
				'>' => return !left || right,
				'=' => return left == right,
				_ => panic!("Invalid operator"),
			}
		}
		Some(NodeValue::Variable(_)) => panic!("Variable in expression"),
		None => panic!("Node has no value"),
	}
}

pub fn replace_variables(node: TreeNodeRef, values: &Vec<bool>, variables: &Vec<char>) -> TreeNodeRef {
	let mut node = node.borrow_mut();
	match node.get_val() {
		Some(NodeValue::Value(_)) => node.clone(),
		Some(NodeValue::Operator('!')) => {
			let left: TreeNodeRef = replace_variables(node.get_left().unwrap(), values, variables);
			node.set_left(left);
			return node.clone();
		}
		Some(NodeValue::Operator(_)) => {
			let left: TreeNodeRef = replace_variables(node.get_left().unwrap(), values, variables);
			let right: TreeNodeRef = replace_variables(node.get_right().unwrap(), values, variables);
			node.set_left(left);
			node.set_right(right);
			return node.clone();
		}
		Some(NodeValue::Variable(variable)) => {
			let index: usize = variables.iter().position(|&v| v == variable).unwrap();
			let value: bool = values[index];
			return TreeNode::new_from(NodeValue::Value(value));
		}
		None => panic!("Node has no value"),
	}
}

pub fn tree_to_rpn(node: TreeNodeRef) -> String {
	let node = node.borrow();
	match node.get_val() {
		Some(NodeValue::Value(value)) => format!("{}", value as u8),
		Some(NodeValue::Variable(variable)) => format!("{}", variable),
		Some(NodeValue::Operator('!')) => {
			let left: String = tree_to_rpn(node.get_left().unwrap());
			format!("{}!", left)
		}
		Some(NodeValue::Operator(operator)) => {
			let left: String = tree_to_rpn(node.get_left().unwrap());
			let right: String = tree_to_rpn(node.get_right().unwrap());
			format!("{}{}{}", left, right, operator)
		}
		None => panic!("Node has no value"),
	}
}
