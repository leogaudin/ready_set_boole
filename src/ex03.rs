use crate::tree::{create_tree, eval_tree, TreeNodeRef};

pub fn eval_formula(formula: &str) -> bool {
	let tree: TreeNodeRef = create_tree(formula);
	return eval_tree(tree);
}
