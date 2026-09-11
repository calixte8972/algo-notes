use crate::common::Node;

struct State {
    // 当前节点第一步向左时的最长长度
    left_len: i32,

    // 当前节点第一步向右时的最长长度
    right_len: i32,

    // 当前节点整棵子树中的最长长度
    subtree_best: i32,
}
fn dfs(node: Node) -> State {
    // 空节点没有路径
    let Some(node) = node else {
        return State {
            left_len: 0,
            right_len: 0,
            subtree_best: 0,
        };
    };

    // 只在这个小作用域中借用节点
    // clone 的只是 Rc 句柄，不是整棵树
    let (left, right) = {
        let current = node.borrow();

        (
            current.left.clone(),
            current.right.clone(),
        )
    };

    // 记录孩子是否存在。
    // 因为下面把 left/right 移动给 dfs 了，
    // 所以需要提前保存 is_some() 的结果。
    let has_left = left.is_some();
    let has_right = right.is_some();

    // 后序 DFS：先计算左右子树
    let left_state = dfs(left);
    let right_state = dfs(right);

    // 当前节点第一步向左，
    // 下一步必须从左孩子向右。
    let left_len = if has_left {
        1 + left_state.right_len
    } else {
        0
    };

    // 当前节点第一步向右，
    // 下一步必须从右孩子向左。
    let right_len = if has_right {
        1 + right_state.left_len
    } else {
        0
    };

    // 最长路径可能完全位于某个子树中，
    // 所以不能只比较 left_len 和 right_len。
    let subtree_best = left_len
        .max(right_len)
        .max(left_state.subtree_best)
        .max(right_state.subtree_best);

    State {
        left_len,
        right_len,
        subtree_best,
    }
}
pub fn longest_zig_zag(root: Node) -> i32 {
    let result = dfs(root);

    result.subtree_best
}