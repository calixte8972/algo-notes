# 解题模式库

模式库用于训练「看到题目特征，就能联想到候选工具」的能力。它不按题号排列，而按可迁移的结构排列。

建议每个模式文件包括：

- **识别信号**：题面里哪些条件值得警觉？
- **维护的信息**：遍历/循环/递归过程中要保存什么状态？
- **不变量**：每一步之后必定仍成立的事实。
- **适用边界**：什么时候不能这样做？
- **代表题**：链接到 `problems/` 中的题目笔记。

可以逐步建立：

```text
patterns/
├── sliding-window.md
├── prefix-sum.md
├── binary-search-answer.md
├── tree-path-state.md
├── graph-traversal.md
└── dynamic-programming-state.md
```
