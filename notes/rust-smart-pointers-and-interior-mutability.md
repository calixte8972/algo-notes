# Rust：`Rc`、`Arc` 与 `RefCell`

> 目标：分清“谁拥有数据”“能否共享”“能否修改”“能否跨线程”这四件事。  
> 关联题目：[1448 · Count Good Nodes in Binary Tree](../problems/1448-count-good-nodes-in-binary-tree.md)、Path Sum III

## 一句话地图

```text
Box<T>         一个拥有者，数据在堆上
Rc<T>          单线程：多个拥有者共享同一份数据
Arc<T>         多线程：多个拥有者共享同一份数据
RefCell<T>     单线程：把借用规则从编译期移到运行期检查
Mutex<T>       多线程：一次只允许一个线程修改数据
RwLock<T>      多线程：多个读者或一个写者
```

它们解决的不是同一个问题：

| 问题 | 常用类型 | 它提供的能力 |
| --- | --- | --- |
| 数据放到堆上，并且只有一个拥有者 | `Box<T>` | 堆分配与唯一所有权 |
| 多个位置要共同拥有同一份数据 | `Rc<T>` / `Arc<T>` | 引用计数式共享所有权 |
| 编译期无法证明借用合法，但程序运行时可以遵守规则 | `RefCell<T>` | 运行时借用检查（内部可变性） |
| 多线程下要安全修改共享数据 | `Mutex<T>` / `RwLock<T>` | 线程同步与互斥访问 |

记忆口诀：

```text
Rc / Arc：谁拥有？能否共享？
RefCell / Mutex / RwLock：谁现在可以读或改？
```

---

## 1. 先理解普通借用规则

正常情况下，Rust 在**编译期**强制以下规则：

```text
要么有任意多个不可变借用：&T
要么只有一个可变借用：&mut T
两者不能同时存在。
```

例如：

```rust
let mut number = 1;
let read = &number;

// number += 1; // 编译错误：read 仍在借用 number
println!("{read}");
```

这样做的好处是：大部分错误在程序运行前就被编译器阻止。

但有些数据结构在编译期很难表达“此刻到底有没有人在借用它”。例如图、带父指针的树、GUI 组件或 LeetCode 的 `Rc<RefCell<TreeNode>>`。这时会用到内部可变性。

---

## 2. `RefCell<T>`：把借用检查放到运行时

`RefCell<T>` 允许通过不可变变量拿到可变访问权；它并没有取消借用规则，而是把检查时间从**编译期**改成**运行期**。

```rust
use std::cell::RefCell;

let score = RefCell::new(0);

*score.borrow_mut() += 10;
assert_eq!(*score.borrow(), 10);
```

这里 `score` 没有写成 `mut`，但仍可以修改里面的 `i32`。这就是“内部可变性”（interior mutability）：外层看起来不可变，内部由 `RefCell` 管理修改。

### 两种借用方法

| 方法 | 返回值 | 含义 |
| --- | --- | --- |
| `borrow()` | `Ref<T>` | 获取只读借用；可同时存在多个 |
| `borrow_mut()` | `RefMut<T>` | 获取可变借用；同一时刻只能有一个 |

`Ref<T>` 与 `RefMut<T>` 是“借用守卫”。只要它们还活着，`RefCell` 就会记住当前存在的借用状态。

```rust
use std::cell::RefCell;

let value = RefCell::new(42);

{
    let read_a = value.borrow();
    let read_b = value.borrow();
    assert_eq!(*read_a + *read_b, 84);
} // read_a、read_b 在这里离开作用域，借用结束

{
    let mut write = value.borrow_mut();
    *write = 100;
} // write 离开作用域，可变借用结束

assert_eq!(*value.borrow(), 100);
```

### 违反规则时会发生什么

以下代码能通过编译，但会在运行时 panic：

```rust,ignore
let cell = RefCell::new(1);
let read = cell.borrow();
let write = cell.borrow_mut(); // panic：已经有不可变借用 read
```

解决方式不是绕过规则，而是缩短前一个借用的生命周期：

```rust
use std::cell::RefCell;

let cell = RefCell::new(1);
let copied_value = *cell.borrow(); // 临时借用在这行结束
*cell.borrow_mut() = copied_value + 1;
assert_eq!(*cell.borrow(), 2);
```

也可以显式调用 `drop(read)`，但更推荐用小作用域 `{ ... }` 或先复制出后续真正需要的值。

### 为什么 `RefCell` 只能在运行时检查？

普通引用将借用关系直接写在类型中：`&T` 是只读借用，`&mut T` 是独占可变借用。编译器因此能在运行前拒绝冲突：

```rust,ignore
let mut value = 1;
let read = &value;
let write = &mut value; // 编译错误：read 仍然活着
```

`RefCell` 的接口不同。无论读取还是修改，它接收的都是外层的 `&self`：

```rust,ignore
cell.borrow();     // 概念上是 RefCell::borrow(&cell)
cell.borrow_mut(); // 概念上也是 RefCell::borrow_mut(&cell)
```

从编译器视角，这两行都只是在取得 `&RefCell<T>` 的不可变借用；多个不可变借用本来完全合法。可 `borrow_mut()` 又希望在只有 `&self` 的情况下修改内部 `T`，这正是内部可变性。编译器无法再通过普通的 `&mut T` 独占规则证明安全性，因此 `RefCell` 必须自己记录实际借用状态。

可以把它想成一个运行时维护的标志：

```text
0       ：当前无人借用
正数 n  ：当前有 n 个只读借用
-1      ：当前有一个可变借用
```

对应操作大致为：

```text
borrow():
    若状态为 -1，失败；否则只读借用数加一。

borrow_mut():
    仅当状态为 0 时成功，并将状态设为 -1；否则失败。

Ref<T> / RefMut<T> 离开作用域：
    自动恢复对应的借用状态。
```

因此，下面的代码可以编译，却会在第二次借用时 panic：

```rust,ignore
let cell = RefCell::new(1);
let read = cell.borrow();
let write = cell.borrow_mut(); // 运行时发现 read 仍然活着
```

这不是编译器能力不足，而是一次设计取舍：借用关系有时依赖运行时分支、输入或复杂的数据结构。普通借用会保守地拒绝所有“可能冲突”的写法；`RefCell` 允许程序继续运行，并在冲突真正发生时检查。代价是错误可能从编译期报错变成运行时 panic。

若希望把失败作为普通分支处理而非 panic，使用 `try_borrow()` 或 `try_borrow_mut()`，它们返回 `Result`。

### 不希望 panic 时

使用 `try_borrow()` 与 `try_borrow_mut()`：

```rust
use std::cell::RefCell;

let cell = RefCell::new(1);

match cell.try_borrow_mut() {
    Ok(mut value) => *value += 1,
    Err(_) => println!("当前仍有借用，暂时不能修改"),
}
```

### `RefCell` 的边界

- 它适合**单线程**场景；它不是线程同步工具。
- 它的借用错误从“编译报错”变成“运行时 panic”，因此不应为了省事而滥用。
- 如果编译器能接受普通的 `&mut T`，优先使用普通借用。

---

## 3. `Rc<T>`：单线程共享所有权

Rust 默认要求每份数据只有一个所有者。普通二叉树可以用 `Box<TreeNode>` 表示，因为每个孩子通常只有一个父节点拥有它。

但当多处都需要“拥有同一个值”时，使用 `Rc<T>`（reference counted，引用计数）。每一次 `Rc::clone` 都只复制一个轻量引用，并把计数加一；它**不会深拷贝底层数据**。

```rust
use std::rc::Rc;

let message = Rc::new(String::from("hello"));
let view_a = Rc::clone(&message);
let view_b = Rc::clone(&message);

assert_eq!(Rc::strong_count(&message), 3);
assert_eq!(message.as_str(), "hello");
assert_eq!(view_a.as_str(), "hello");
assert_eq!(view_b.as_str(), "hello");
```

概念上是：

```text
message ──┐
view_a  ──┼──► 同一份 String("hello")
view_b  ──┘
```

最后一个 `Rc` 离开作用域时，引用计数归零，底层数据才会被释放。

### 为什么不是普通 `.clone()`？

对于 `Rc<T>`：

```rust
let another = Rc::clone(&original);
```

表达的是“我想要同一份数据的另一个拥有权”。这比 `original.clone()` 更清楚地提醒读者：发生的是引用计数增加，不是 `T` 的深拷贝。

### `Rc` 的边界

- `Rc<T>` 只能在**单线程**中使用；它不是 `Send` 或 `Sync`。
- `Rc<T>` 提供共享所有权，但默认只能读取 `T`；不能直接修改内部数据。
- 需要“共享 + 修改”时，经常组合为 `Rc<RefCell<T>>`。

### 强引用循环

如果 A 强引用 B，B 又强引用 A，双方引用计数都不会归零，造成内存泄漏。对于“子节点指向父节点”这类反向链接，应使用 `Weak<T>`：

```text
父 ── strong Rc ──► 子
父 ◄── weak Weak ── 子
```

`Weak<T>` 不计入强引用数量；要使用时通过 `upgrade()` 尝试变回 `Rc<T>`，目标已释放时会得到 `None`。

---

## 4. `Arc<T>`：多线程共享所有权

`Arc<T>`（atomically reference counted）与 `Rc<T>` 的所有权模型相同，区别在于它用原子操作维护引用计数，因此能安全地跨线程共享。

```rust
use std::sync::Arc;
use std::thread;

let message = Arc::new(String::from("hello"));
let worker_message = Arc::clone(&message);

let worker = thread::spawn(move || {
    assert_eq!(worker_message.as_str(), "hello");
});

worker.join().unwrap();
assert_eq!(message.as_str(), "hello");
```

### `Arc` 不等于“可以修改”

`Arc<T>` 只解决“多个线程共同拥有数据”，不自动允许多线程修改数据。需要共享修改时，通常组合 `Arc<Mutex<T>>`：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let total = Arc::new(Mutex::new(0));
let worker_total = Arc::clone(&total);

let worker = thread::spawn(move || {
    *worker_total.lock().unwrap() += 1;
});

worker.join().unwrap();
assert_eq!(*total.lock().unwrap(), 1);
```

其中：

```text
Arc   ：两个线程共同拥有 total
Mutex ：同一时刻只允许一个线程拿到可变访问权
```

读多写少的共享数据也可以考虑 `Arc<RwLock<T>>`：它允许多个读者同时读取，但写入者必须独占。

### `Arc` 的边界

- 原子引用计数有比 `Rc` 更高的开销；单线程不要因为“看起来更通用”而使用 `Arc`。
- `Arc<RefCell<T>>` 不能用于多线程；`RefCell` 不具备跨线程同步能力。
- `Rc<Mutex<T>>` 也不能跨线程；问题在 `Rc` 本身不能在线程间移动。
- 常见多线程可变共享组合是 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`。

---

## 5. 为什么 LeetCode 二叉树常用 `Rc<RefCell<TreeNode>>`

LeetCode Rust 模板通常定义：

```rust
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
```

它是一个通用模板，既能支持只读树题，也能支持修改树的题。

| 成分 | 在二叉树里的含义 |
| --- | --- |
| `Option` | 孩子可能不存在 |
| `Rc` | 可以保存、传递多个指向同一节点的句柄 |
| `RefCell` | 需要改节点或孩子指针时可在运行时借用检查 |

对于 Path Sum III、Good Nodes 这种只读题：

```text
算法本身只需要读节点值、访问左右孩子。
因此实际只调用 borrow()，不调用 borrow_mut()。
```

常见安全写法是尽快从借用中复制出后续需要的值：

```rust
let (value, left, right) = {
    let current = node.borrow();
    (
        current.val,
        current.left.clone(),
        current.right.clone(),
    )
};

// current 的借用已经结束；现在可安全递归 left 和 right。
```

这里 `left.clone()`、`right.clone()` 复制的是 `Rc` 引用，不会复制整棵子树。

如果是自己从零设计一个“没有共享节点、没有父指针”的二叉树，用 `Box<TreeNode>` 往往更简单；LeetCode 中则应遵从题目已经给定的 `Rc<RefCell<TreeNode>>` 类型。

---

## 6. 选择速查表

| 需求 | 推荐类型 | 原因 |
| --- | --- | --- |
| 普通树、链表，结构有唯一拥有者 | `Box<T>` | 最简单，编译期借用检查 |
| 单线程，只需多处读同一数据 | `Rc<T>` | 共享所有权，无原子开销 |
| 单线程，多处共享且需要修改 | `Rc<RefCell<T>>` | `Rc` 管所有权，`RefCell` 管运行时借用 |
| 多线程，多处读同一数据 | `Arc<T>` | 原子引用计数保证线程安全共享 |
| 多线程，多处共享且需要修改 | `Arc<Mutex<T>>` | 共享所有权 + 独占修改 |
| 多线程，读多写少 | `Arc<RwLock<T>>` | 多读者并发、写者独占 |

可以按下面顺序决策：

```text
需要多个拥有者吗？
├── 不需要：Box<T> 或普通 T / &T / &mut T
└── 需要：要跨线程吗？
    ├── 不跨线程：Rc<T>
    │   └── 还需要修改吗？是 → Rc<RefCell<T>>
    └── 跨线程：Arc<T>
        └── 还需要修改吗？是 → Arc<Mutex<T>> / Arc<RwLock<T>>
```

---

## 7. 高频错误与检查清单

### 错误 1：把 `Rc::clone` 当成深拷贝

`Rc::clone(&node)` 只增加引用计数。若真的要复制底层节点数据，需要 `TreeNode: Clone` 并克隆 `TreeNode` 本身；两者成本与含义完全不同。

### 错误 2：在 `borrow()` 还活着时递归或 `borrow_mut()`

将读取孩子的逻辑限制在一个小作用域内，先取出 `left`、`right` 再递归。这样借用何时结束一目了然。

### 错误 3：为了修改而无条件套 `RefCell`

先问：能否在构建结构时修改一次，之后用普通 `&mut`？能做到就通常更安全、更简单。

### 错误 4：误以为 `Arc` 自动同步数据访问

`Arc` 只同步**引用计数**，不保护 `T` 的内部状态。共享可变数据仍需 `Mutex`、`RwLock` 或原子类型。

### 错误 5：父子节点都用强 `Rc`

反向父指针应使用 `Weak`，否则可能形成强引用循环而泄漏。

### 自检问题

1. 我的难题是多个所有者，还是多个借用者？
2. 数据会跨线程吗？
3. 数据真的需要在共享后修改吗？
4. 编译期借用是否已经足够？
5. 是否存在指回上层的链接，导致 `Rc` 循环？

---

## 8. 与算法题的联系

算法题先关注“状态和遍历”，再处理 Rust 的节点类型：

```text
第一步：递归/迭代函数需要维护什么算法状态？
第二步：节点是只读还是需要修改？
第三步：根据平台给定的 TreeNode 类型安全取得 val、left、right。
```

不要把 `Rc<RefCell<TreeNode>>` 误认为算法难点。它通常只是 Rust 表达树节点的方式；真正的题目核心仍可能是 DFS、前缀和、动态规划或图遍历。
