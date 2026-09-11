//! LeetCode 437: Path Sum III.
//!
//! 解题笔记：[problems/437-path-sum-iii.md](../../problems/437-path-sum-iii.md)

use crate::common::Node;
use std::collections::HashMap;

/// 统计所有向下路径中，节点值之和等于 target_sum 的路径数量。
///
/// 使用前缀和哈希表，将任意起点的路径查询合并到一次 DFS 中。
pub fn path_sum(root: Node, target_sum: i32) -> i32 {
    fn dfs(
        node: Node,
        prefix_sum: i64,
        target_sum: i64,
        prefix_sums: &mut HashMap<i64, i32>,
    ) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let (value, left, right) = {
            let node = node.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };

        let current_sum = prefix_sum + value as i64;
        let paths_ending_here = prefix_sums
            .get(&(current_sum - target_sum))
            .copied()
            .unwrap_or_default();

        *prefix_sums.entry(current_sum).or_default() += 1;

        let paths_in_subtrees = dfs(left, current_sum, target_sum, prefix_sums)
            + dfs(right, current_sum, target_sum, prefix_sums);

        let should_remove = {
            let count = prefix_sums
                .get_mut(&current_sum)
                .expect("current prefix sum must be present during rollback");
            *count -= 1;
            *count == 0
        };
        if should_remove {
            prefix_sums.remove(&current_sum);
        }

        paths_ending_here + paths_in_subtrees
    }

    let mut prefix_sums = HashMap::from([(0_i64, 1_i32)]);
    dfs(root, 0, target_sum as i64, &mut prefix_sums)
}

#[cfg(test)]
mod tests {
    use super::path_sum;
    use crate::common::{Node, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32, left: Node, right: Node) -> Node {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn counts_paths_from_the_example() {
        let root = node(
            10,
            node(
                5,
                node(3, node(3, None, None), node(-2, None, None)),
                node(2, None, node(1, None, None)),
            ),
            node(-3, None, node(11, None, None)),
        );

        assert_eq!(path_sum(root, 8), 3);
    }

    #[test]
    fn counts_overlapping_zero_sum_paths() {
        let root = node(0, node(0, None, None), node(0, None, None));

        assert_eq!(path_sum(root, 0), 5);
    }

    #[test]
    fn handles_negative_values_and_empty_trees() {
        let root = node(1, node(-1, node(1, None, None), None), None);

        assert_eq!(path_sum(root, 0), 2);
        assert_eq!(path_sum(None, 0), 0);
    }
}
