//! LeetCode 1448: Count Good Nodes in Binary Tree.
//!
//! 解题笔记：[`problems/1448-count-good-nodes-in-binary-tree.md`](../../problems/1448-count-good-nodes-in-binary-tree.md)

pub use crate::common::{Node, TreeNode};

/// 统计从根到节点路径上值不小于所有祖先的节点数量。
pub fn good_nodes(root: Node) -> i32 {
    fn dfs(node: Node, path_max: i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let node = node.borrow();
        let current_count = i32::from(node.val >= path_max);
        let next_max = path_max.max(node.val);

        current_count + dfs(node.left.clone(), next_max) + dfs(node.right.clone(), next_max)
    }

    dfs(root, i32::MIN)
}

#[cfg(test)]
mod tests {
    use super::good_nodes;
    use crate::common::{Node, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32, left: Node, right: Node) -> Node {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn counts_the_example_tree() {
        let root = node(
            3,
            node(1, node(3, None, None), None),
            node(4, node(1, None, None), node(5, None, None)),
        );

        assert_eq!(good_nodes(root), 4);
    }

    #[test]
    fn equal_to_path_maximum_is_good() {
        let root = node(5, node(3, node(5, None, None), None), None);

        assert_eq!(good_nodes(root), 2);
    }

    #[test]
    fn empty_tree_has_no_good_nodes() {
        assert_eq!(good_nodes(None), 0);
    }
}
