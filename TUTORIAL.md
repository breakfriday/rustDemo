# Rust wgpu Demo 入门教程

这个项目是一个最小可运行的 Rust 图形渲染 demo。它使用 `winit` 创建窗口，使用 `wgpu` 调用 GPU，在窗口里绘制一个彩色三角形。

## 项目结构

```text
.
├── Cargo.toml
├── Cargo.lock
├── src
│   └── main.rs
└── TUTORIAL.md
```

文件说明：

- `Cargo.toml`：项目配置文件，声明包名、版本、Rust edition 和依赖包。
- `Cargo.lock`：依赖锁定文件，由 Cargo 自动生成，通常不需要手动修改。
- `src/main.rs`：二进制程序入口文件，程序从 `fn main()` 开始执行。
- `target/`：编译输出目录，运行 `cargo build` 后自动生成。

## Cargo 是什么

`cargo` 是 Rust 官方项目管理工具。它负责创建项目、下载依赖、编译、运行、测试和格式化代码。

常用命令：

```bash
cargo init --bin .
```

在当前目录初始化一个可执行 Rust 项目。

```bash
cargo new my_app
```

创建一个新的 Rust 可执行项目目录。

```bash
cargo check
```

检查代码是否能通过编译，但不生成最终可执行文件。开发时很常用，速度比完整编译更快。

```bash
cargo run
```

编译并运行项目。

```bash
cargo build
```

编译 debug 版本。

```bash
cargo build --release
```

编译 release 版本，适合正式运行。

```bash
cargo fmt
```

格式化代码。

## 编译和运行

调试运行：

```bash
cargo run
```

编译 debug 可执行文件：

```bash
cargo build
```

输出文件：

```text
target/debug/wgpu-demo
```

编译 release 可执行文件：

```bash
cargo build --release
```

输出文件：

```text
target/release/wgpu-demo
```

运行 release 文件：

```bash
./target/release/wgpu-demo
```

## Cargo.toml 配置

当前项目的核心配置在 `Cargo.toml`：

```toml
[package]
name = "wgpu-demo"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1"
env_logger = "0.11"
log = "0.4"
pollster = "0.4"
wgpu = "29.0.1"
winit = "0.30.13"
```

`[package]` 是项目自身信息：

- `name`：项目名称，也决定默认可执行文件名。
- `version`：项目版本。
- `edition`：Rust 语法版本，这里使用 `2024`。

`[dependencies]` 是依赖包列表：

- `wgpu`：跨平台 GPU 渲染 API。
- `winit`：跨平台窗口和事件循环库。
- `pollster`：把 async 初始化代码同步执行。
- `anyhow`：简化错误处理。
- `log`：日志接口。
- `env_logger`：日志输出实现。

## 增加依赖包

推荐使用 `cargo add`：

```bash
cargo add rand
```

Cargo 会自动修改 `Cargo.toml`：

```toml
[dependencies]
rand = "0.9"
```

然后可以在代码里使用：

```rust
use rand::random;

fn main() {
    let value: u32 = random();
    println!("{value}");
}
```

也可以手动编辑 `Cargo.toml`：

```toml
[dependencies]
rand = "0.9"
```

然后运行：

```bash
cargo check
```

Cargo 会自动下载并编译依赖。

## 安装依赖包

Rust 项目通常不需要单独执行类似 `npm install` 的命令。

当你执行下面任意命令时：

```bash
cargo check
cargo build
cargo run
```

Cargo 会自动完成这些事情：

1. 读取 `Cargo.toml`。
2. 解析依赖版本。
3. 下载缺失的 crate。
4. 生成或更新 `Cargo.lock`。
5. 编译依赖。
6. 编译当前项目。

依赖会缓存在本机 Cargo 目录中，项目自己的编译产物会放在 `target/` 目录。

## 入口文件

Rust 可执行项目默认入口文件是：

```text
src/main.rs
```

入口函数是：

```rust
fn main() {
    println!("Hello, world!");
}
```

当前项目的入口函数是：

```rust
fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut App::default())?;
    Ok(())
}
```

这里的含义：

- `env_logger::init()`：初始化日志输出。
- `EventLoop::new()?`：创建窗口事件循环。
- `event_loop.run_app(...)`：启动应用。
- `Ok(())`：程序正常结束。
- `?`：如果前面的操作失败，就提前返回错误。

## 多入口文件

默认情况下只有一个入口：

```text
src/main.rs
```

如果你想创建多个可执行程序，可以使用：

```text
src/bin/demo1.rs
src/bin/demo2.rs
```

运行指定入口：

```bash
cargo run --bin demo1
cargo run --bin demo2
```

## demo 运行流程

这个项目的简化运行流程：

```text
main()
  ↓
创建 EventLoop
  ↓
启动 App
  ↓
resumed() 创建窗口
  ↓
初始化 wgpu
  ↓
窗口请求重绘
  ↓
render() 绘制三角形
```

核心应用结构：

```rust
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    gpu_state: Option<GpuState>,
}
```

`App` 负责窗口生命周期和事件处理。

GPU 状态结构：

```rust
struct GpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    size: PhysicalSize<u32>,
}
```

字段说明：

- `surface`：窗口上的绘图表面。
- `device`：GPU 设备。
- `queue`：提交 GPU 命令的队列。
- `config`：绘图表面的格式、宽高、刷新策略等配置。
- `render_pipeline`：渲染管线，描述如何把顶点变成屏幕上的像素。
- `size`：当前窗口大小。

## Shader

项目中的 shader 直接写在 `src/main.rs` 里：

```rust
const SHADER: &str = r#"
...
"#;
```

它使用 WGSL 语言。

顶点着色器：

```wgsl
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    ...
}
```

它决定三角形三个顶点的位置。

片元着色器：

```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

它决定三角形每个像素的颜色。

## Rust 基础概念

### 变量默认不可变

```rust
let x = 1;
```

`x` 默认不能再修改。

如果需要修改，使用 `mut`：

```rust
let mut x = 1;
x = 2;
```

### 函数返回值

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

最后一行没有分号，表示这个表达式就是返回值。

### 结构体

```rust
struct User {
    name: String,
    age: u32,
}
```

创建结构体：

```rust
let user = User {
    name: String::from("Alice"),
    age: 20,
};
```

### impl 方法

```rust
impl User {
    fn say_hello(&self) {
        println!("hello {}", self.name);
    }
}
```

`&self` 表示只读借用当前对象。

`&mut self` 表示可变借用当前对象：

```rust
impl GpuState {
    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }
}
```

### Result 和错误处理

```rust
fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    Ok(())
}
```

`Result` 表示函数可能成功，也可能失败。

`?` 表示：

- 如果成功，取出成功值。
- 如果失败，直接返回错误。

## 可以练习的修改

### 修改窗口标题

在 `src/main.rs` 里找到：

```rust
.with_title("wgpu demo")
```

改成：

```rust
.with_title("我的第一个 Rust wgpu 程序")
```

然后运行：

```bash
cargo run
```

### 修改背景颜色

找到：

```rust
wgpu::Color {
    r: 0.08,
    g: 0.09,
    b: 0.12,
    a: 1.0,
}
```

修改 `r`、`g`、`b` 的值。取值范围通常是 `0.0` 到 `1.0`。

### 修改三角形颜色

找到 shader 里的：

```wgsl
var colors = array<vec3<f32>, 3>(
    vec3<f32>(1.0, 0.2, 0.2),
    vec3<f32>(0.2, 1.0, 0.4),
    vec3<f32>(0.2, 0.4, 1.0),
);
```

修改三个颜色值。

### 修改三角形位置

找到：

```wgsl
var positions = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 0.6),
    vec2<f32>(-0.6, -0.6),
    vec2<f32>(0.6, -0.6),
);
```

坐标范围大致是：

```text
x: -1.0 到 1.0
y: -1.0 到 1.0
```

可以尝试改大、改小、移动三角形位置。

## 常用命令速查

```bash
cargo check
```

检查代码。

```bash
cargo run
```

编译并运行。

```bash
cargo build
```

编译 debug 版本。

```bash
cargo build --release
```

编译 release 版本。

```bash
cargo fmt
```

格式化代码。

```bash
cargo add 包名
```

添加依赖包。

```bash
cargo tree
```

查看完整依赖树。

## 学习建议

建议先从下面几个小改动开始：

1. 改窗口标题。
2. 改背景颜色。
3. 改三角形颜色。
4. 改三角形顶点位置。
5. 新增一个依赖包，例如 `rand`。
6. 在窗口标题或日志里使用随机数。

每次修改后运行：

```bash
cargo check
cargo run
```

先保证能编译，再观察运行效果。
