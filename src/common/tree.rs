
use std::cell::RefCell;
use std::rc::Rc;
pub type Node = Option<Rc<RefCell<TreeNode>>>;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
