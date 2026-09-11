//! LeetCode 风格的共享二叉树类型。

use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;

/// 与 LeetCode 题目中的二叉树节点定义保持一致。
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
