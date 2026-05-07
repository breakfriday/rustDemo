# Rust 项目在 VS Code 中的配置说明

## 1. VS Code 需要安装哪些插件

Rust 项目里，核心插件是这几个：

- `rust-lang.rust-analyzer`
- `vadimcn.vscode-lldb`
- `tamasfe.even-better-toml`

它们的作用分别是：

### `rust-lang.rust-analyzer`

这是 Rust 开发最核心的插件，负责：

- 代码提示
- 自动补全
- 跳转定义
- 查找引用
- 类型推导
- 错误诊断
- 重命名
- 悬停查看类型和文档

如果只装一个插件，那就是它。

### `vadimcn.vscode-lldb`

这是 Rust 调试插件，负责：

- 打断点
- 单步调试
- 查看变量
- 调试 `cargo run`
- 调试 Rust 二进制程序

### `tamasfe.even-better-toml`

这个插件主要服务于 `Cargo.toml` 和其他 `.toml` 配置文件，负责：

- TOML 语法高亮
- TOML 补全
- 配置错误检查
- `Cargo.toml` 编辑体验增强
- TOML 格式化

它不是 Rust 代码补全的核心插件，但对 Rust 项目很实用，因为 `Cargo.toml` 会经常改。

## 2. Rust 工具链建议安装的组件

除了 VS Code 插件，还建议安装这些 Rust 组件：

```bash
rustup component add rust-src rustfmt clippy
```

作用分别是：

- `rust-src`
  - 让 `rust-analyzer` 更好地支持标准库跳转和类型分析
- `rustfmt`
  - 代码格式化工具
- `clippy`
  - 更严格的静态检查工具

## 3. VS Code 配置文件的作用

项目里生成了以下 VS Code 配置：

- `.vscode/settings.json`
- `.vscode/launch.json`
- `.vscode/tasks.json`

### `settings.json`

用于配置编辑器行为和 `rust-analyzer`。

当前重点配置包括：

- 不启用 `formatOnSave`
- 使用 `rust-analyzer` 做 Rust 语言分析
- 启用 `clippy` 作为检查命令
- 启用 build script 支持
- 启用 proc macro 支持
- 启用 inlay hints（类型提示、参数提示等）

### `launch.json`

用于配置调试。

例如可以配置：

- 调试主程序 `wgpu-demo`
- 调试 `a.rs` 对应的二进制

### `tasks.json`

用于配置 VS Code 任务，把常用 Cargo 命令做成可直接运行的任务。

典型任务包括：

- `cargo check`
- `cargo clippy`
- `cargo test`
- `cargo run`

使用方式：

1. 打开命令面板 `Ctrl+Shift+P`
2. 输入 `Tasks: Run Task`
3. 选择任务执行

作用：

- 不用每次手动敲命令
- 错误会显示在 Problems 面板
- 可点击错误直接跳到对应代码

## 4. 为什么 `main.rs` 有提示，根目录 `a.rs` 没有提示

这是因为 Rust 和 Cargo 识别源码文件，不是“目录里有这个文件就算项目的一部分”，而是必须满足以下条件之一：

### 方式 1：处于 Cargo 默认约定位置

Cargo 默认会识别这些二进制入口：

- `src/main.rs`
- `src/bin/*.rs`

所以：

- `src/main.rs` 会自动被识别为主程序
- `src/bin/a.rs` 会自动被识别为一个独立二进制

### 方式 2：在 `Cargo.toml` 中显式声明

如果文件不在默认位置，比如根目录下的 `a.rs`，那就需要手动声明：

```toml
[[bin]]
name = "a"
path = "a.rs"
```

这样 Cargo 才知道：

- `a.rs` 是一个可执行程序
- `rust-analyzer` 也才会把它纳入工程分析范围

如果不声明，它只是一个普通文件，不属于 Cargo target，因此通常只有基础语法高亮，没有完整的补全、跳转、诊断。

## 5. 为什么 `main.rs` 不需要写 `[[bin]]`

因为 `src/main.rs` 是 Cargo 的默认入口位置。

也就是说，即使 `Cargo.toml` 里不写：

```toml
[[bin]]
name = "wgpu-demo"
path = "src/main.rs"
```

Cargo 也会自动识别它。

而根目录 `a.rs` 不在 Cargo 默认扫描位置里，所以如果它不放到 `src/bin/`，就必须显式写：

```toml
[[bin]]
name = "a"
path = "a.rs"
```

所以本质不是：

- `a.rs` 必须放到 `bin` 目录

而是：

- `a.rs` 必须被 Cargo 识别成一个二进制 target

## 6. `path` 路径是怎么查找的

在 `Cargo.toml` 的 `[[bin]]` 中：

```toml
[[bin]]
name = "a"
path = "a.rs"
```

这里的 `path` 是相对于 `Cargo.toml` 所在目录解析的。

例如目录结构：

```text
rust/
  Cargo.toml
  a.rs
  src/
    main.rs
```

那么：

- `path = "a.rs"` 指向 `rust/a.rs`
- `path = "src/main.rs"` 指向 `rust/src/main.rs`

不是相对于 `src/`，也不是相对于当前文件，而是相对于 `Cargo.toml`。

## 7. `cargo run` 为什么默认能运行 `main.rs`

因为当项目只有一个二进制目标时，`cargo run` 可以直接运行它。

如果项目里只有：

- `src/main.rs`

那么 `cargo run` 默认就运行这个主程序。

但如果项目里有多个二进制目标，例如：

- `src/main.rs`
- `a.rs`（通过 `[[bin]]` 显式声明）
- 或者 `src/bin/*.rs`

那么 `cargo run` 默认就可能不明确，需要指定运行哪个：

```bash
cargo run --bin wgpu-demo
cargo run --bin a
```

## 8. `default-run` 的作用是什么

在 `Cargo.toml` 中：

```toml
[package]
name = "wgpu-demo"
version = "0.1.0"
edition = "2024"
default-run = "wgpu-demo"
```

这里的：

```toml
default-run = "wgpu-demo"
```

表示：

- 当执行 `cargo run` 时
- 默认运行名字为 `wgpu-demo` 的二进制 target

它不是直接写“默认跑 `main.rs` 文件”，而是指定“默认跑哪个 bin target”。

只不过在当前项目里：

- `src/main.rs` 自动对应的 bin 名就是 `wgpu-demo`

所以看起来等价于“默认跑 `main.rs`”。

更准确地说：

- `main.rs` 是默认入口位置
- `default-run` 选择的是默认执行的 target 名

## 9. 当前项目的 Cargo 配置含义

当前核心配置可以理解为：

```toml
[package]
name = "wgpu-demo"
version = "0.1.0"
edition = "2024"
default-run = "wgpu-demo"

[[bin]]
name = "a"
path = "a.rs"
```

含义是：

- 包名叫 `wgpu-demo`
- 默认主二进制是 `src/main.rs`
- 根目录 `a.rs` 被显式声明为一个 bin target
- `cargo run` 默认运行 `wgpu-demo`
- `cargo run --bin a` 运行 `a.rs`

## 10. 用 `[[bin]]` 做显式管理是不是更好

结论是：

- 小项目、单入口项目：`src/main.rs` 最自然，最符合 Cargo 默认约定
- 多入口项目：`[[bin]]` 更显式、更语义化、扩展性更高
- 实际工程里最常见的平衡方案是：
  - 主程序放 `src/main.rs`
  - 其他独立程序放 `src/bin/*.rs`

如果希望结构完全显式，也可以统一写成：

```toml
[[bin]]
name = "wgpu-demo"
path = "src/main.rs"

[[bin]]
name = "a"
path = "a.rs"
```

这种写法的优点是：

- 所有入口都在 `Cargo.toml` 里集中声明
- 更利于扩展多 bin 项目
- 更利于团队理解项目入口结构

## 11. 本次结论

这次的关键结论有三点：

1. Rust 文件要想在 VS Code 里获得完整补全、跳转、诊断，必须被 Cargo 识别为工程的一部分。
2. `src/main.rs` 和 `src/bin/*.rs` 是 Cargo 默认识别的位置，根目录普通 `.rs` 文件不会自动识别。
3. 根目录 `a.rs` 也可以正常参与工程分析，只要在 `Cargo.toml` 中通过 `[[bin]]` 显式声明。

## 12. 当前建议的使用方式

常用命令建议如下：

```bash
cargo run
cargo run --bin wgpu-demo
cargo run --bin a
cargo check
cargo clippy
cargo test
```

推荐理解方式：

- `cargo run`
  - 跑默认目标，由 `default-run` 决定
- `cargo run --bin xxx`
  - 明确跑指定二进制
- `[[bin]]`
  - 显式声明一个二进制目标
- `src/main.rs`
  - Cargo 默认主入口
- `src/bin/*.rs`
  - Cargo 默认附加二进制目录
