# 知识笔记

这里写「一类题共享的知识」，不要把某一道具体题的完整过程放在这里。

建议按主题建立 Markdown 文件，例如：

```text
notes/
├── arrays-and-strings.md
├── linked-list.md
├── binary-tree.md
├── graph.md
├── dynamic-programming.md
└── rust-for-algorithms.md
```

每篇专题笔记建议回答：

1. 这类问题在问什么？
2. 常见输入特征或关键词是什么？
3. 有哪些可选方法，各自的适用条件是什么？
4. 最易错的边界在哪里？
5. 链接哪些代表题目？

新知识优先写成自己的语言；必要时再补一个小例子验证理解。

当前专题：

- [Rust：`Rc`、`Arc` 与 `RefCell`](rust-smart-pointers-and-interior-mutability.md)：共享所有权、内部可变性、多线程同步，以及 LeetCode 二叉树类型的阅读方式。
- [二叉树基础](binary-tree-basics.md)：术语、深度与高度、树的类型、表示方式、DFS/BFS 与树题检查清单。
- [二叉树遍历：前序、中序、后序](binary-tree-traversal-orders.md)：访问时机、递归调用栈、Rust 骨架和树题选型。
