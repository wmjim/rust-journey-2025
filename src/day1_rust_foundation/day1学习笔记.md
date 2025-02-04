## 目标

- [x] 掌握 Rust 语法基础（变量、数据类型、函数、控制流）
- [x] 了解 Rust 开发环境，学会使用 Cargo
- [x] 完成第一天的基本代码与测试
- [x] 上传 Git 代码，并提交 Markdown 笔记
- [x] 制作 Anki 记忆卡

## 任务1：Rust 开发环境搭建

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 检查安装
rustup --version
cargo --version

cargo new task # 创建一个 Rust 项目
```

## 任务2：Rust 语法基础

### 2.1 变量与数据类型

* 定义一个不可变变量：`let x = 5;`
* 定义一个可变变量：`let mut y = 10;`
* 探索 `i32`, `f64`, `bool`, `char`, `String` 数据类型。
* **问题**：如何打印这些变量？

```rust
pub fn print_variables() {
    // 定义一个不可变变量
    let x = 5;
    // 定义一个可变变量
    let mut y = 10;
    println!("x: {}, y: {}", x, y); // 使用 println! 宏打印变量
    y = 20;
    println!("updated y: {}", y);
}
```

### 2.2 函数

* 定义一个计算 **两个数的最大值** 的函数。
* 定义一个 **递归函数** 来计算斐波那契数列的第 N 项。

> 每个数等于前两个数之和：
> 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...

```rust
pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
}
```

### 2.3 控制流

* 使用 `if/else` 判断一个数是否是正数、负数或零。
* 使用 `for` 循环计算 **1 到 100 的和**。
* 使用 `while` 循环打印出 **斐波那契数列**。

```rust
use super::functions;

fn check_number(num: i32) {
    if num < 0 {
        println!("Negative");
    } else if num == 0 {
        println!("Zero");
    } else {
        println!("Positive");
    }
}

fn sum() -> u32 {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    sum
}

fn while_fibonacci(n: i32) {
    while n > 0 {
        println!("{} ", functions::fibonacci(n));
    }
    println!("");
}
```

### 2.4 模式匹配

* 使用 `match` 来 **判断星期几**，基于数字输入返回星期几的名称。
* 使用 `match` 语法处理 HTTP 状态码。

```rust
pub fn day_of_week(day: u8) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day"
    }
}
```


## 任务3：实现基本的 CLI 任务管理器

**实现 CLI 输入：**
- 使用 `std::env::args()` 来接收命令行参数。
- 基于命令：`add "Task"` 或 `list`，实现不同的功能。

### `std::env::args()` 介绍

在 Rust 中 `std::env::args()` 用于获取命令行参数，它返回一个 `std::env::Args` 迭代器，该迭代器包含程序的所有命令行参数（包括程序的名称）。

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Received arguments: {:?}", args);
}
```

编译后运行和输出：

```bash
./my_program arg1 arg2 arg3
Received arguments: ["./my_program", "arg1", "arg2", "arg3"]
```

### `as_str()` 方法介绍

在 Rust 中，`as_str()` 是 `String` 类型上的一个方法，用于获取 `&str` （字符串切片）的引用。它的作用是将 `String` 类型转换为 `&str`，避免不必要的复制，提升性能。

```rust
let s = String::from("Hello, Rust!");
let s_slice: &str = s.as_str(); // 更推荐，它表达了获取 `&str` 的意图
let slice2: &str = &s[..]; // 也能转换为 &str
```

### 实现方案

```rust
use std::env;

fn day1_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "add" => add_task(&args[2]),
            "list" => list_task(),
            _ => println!("Invalid command"),
        }
    }
}

fn add_task(task: &str) {
    println!("Task added: {}", task);
}

fn list_task() {
    println!("Task list: ");
    println!("1. Learn Rust.");
}
```
