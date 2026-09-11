# algo-notes

这是一个以 **主动思考、可复习的笔记、可运行的 Rust 实现** 为核心的算法学习仓库。

它不是题解收集夹。每一道题都应该留下你的推理过程：你最初怎么理解、卡在哪里、哪条提示让你突破、以后怎样更快识别同类题。

## 使用方式

每道新题按下面顺序进行：

1. 在 `problems/` 新建题目笔记，复制 [`templates/problem.md`](templates/problem.md)。
2. 先独立完成笔记中的「不看提示思考」部分；建议至少给自己 15–25 分钟。
3. 需要帮助时，只取一层提示；不要直接看完整解法。提示层级见 [求助约定](#求助约定)。
4. 能清楚说明状态、选择与转移/遍历逻辑后，再在 `src/problems/` 编写 Rust 实现与测试。
5. 做完后补上复杂度、踩坑和复习时间。复习时优先重做，而不是重读答案。

## 目录说明

```text
algo-notes/
├── notes/                  # 知识笔记：概念、专题、易错点
├── problems/               # 每题的思考过程与复盘（Markdown）
├── patterns/               # 可迁移的解题模式与识别信号
├── templates/              # 新题和复盘模板
├── src/                    # 可运行、可测试的 Rust 实现
│   ├── common/             # 通用辅助类型（按需添加）
│   └── problems/           # 一题一个模块
├── Cargo.toml
└── README.md
```

## 求助约定

以后你可以直接这样对我说：

- `只问我问题，不给提示`：我会用问题帮你检查理解与推理。
- `给第一层提示`：只指出该关注什么信息，不说算法名或完整步骤。
- `再给一层提示`：给一个关键观察或状态设计方向。
- `帮我检查思路`：你先说方案，我检查正确性、边界和复杂度。
- `给伪代码`：只在你已有思路后整理步骤，不直接给 Rust 实现。
- `给代码`：只有你明确提出时，我才提供完整实现。

默认情况下，我会采用前四种方式引导你。

## 命名约定

- 题目笔记：`problems/<平台编号>-<英文短名>.md`，例如 `problems/1448-count-good-nodes-in-binary-tree.md`。
- Rust 模块：`src/problems/p<编号>_<英文短名>.rs`，例如 `src/problems/p1448_count_good_nodes.rs`。
- 每个题目笔记都应链接到对应 Rust 文件；每个 Rust 文件的模块文档都应链接回题目笔记。

## 建议的复习节奏

完成当天记为第 0 天；之后可在第 1、3、7、14、30 天尝试不看笔记重做。若不能独立写出核心思路，就把题目退回 `problems/` 中的「待复习」状态。

开始时可以先阅读：

- [题目笔记说明](problems/README.md)
- [专题笔记说明](notes/README.md)
- [模式库说明](patterns/README.md)
- [新题模板](templates/problem.md)
- [完整样例：1448 · Count Good Nodes in Binary Tree](problems/1448-count-good-nodes-in-binary-tree.md)
