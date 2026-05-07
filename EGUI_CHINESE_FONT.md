# egui 中文字体问题和内置字体方案

## 问题现象

在 `eframe/egui` 程序里写中文文本时，中文可能显示成小方块：

```rust
ui.heading("简单 eframe UI Demo");
ui.label("这是一个基于 egui/eframe 的桌面窗口示例。");
```

这通常不是 Rust 源码编码错误，也不是字符串乱码，而是字体缺少中文字形。

## 根本原因

`egui` 默认内置字体主要覆盖拉丁字符，不完整覆盖中文、日文、韩文等 CJK 字符。

当 UI 文本里出现中文时，如果 `egui` 当前字体找不到对应 glyph，就会把字符渲染成方块。

这个问题不是 Ubuntu 独有。Windows 和 macOS 虽然通常自带中文字体，但 `egui` 不会自动把系统中文字体注册进自己的字体系统。要稳定显示中文，应该显式注册中文字体。

## 推荐方案：内置中文字体

跨平台分发时，推荐把中文字体文件放进项目里，然后用 `include_bytes!` 编译进二进制。

本项目使用：

```text
assets/fonts/wqy-microhei.ttc
```

代码示例：

```rust
use eframe::egui;
use std::sync::Arc;

fn configure_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "chinese_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/wqy-microhei.ttc"
        ))),
    );

    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts
            .families
            .entry(family)
            .or_default()
            .insert(0, "chinese_font".to_owned());
    }

    ctx.set_fonts(fonts);
}
```

调用位置：

```rust
eframe::run_native(
    "eframe UI Demo",
    options,
    Box::new(|cc| {
        configure_chinese_fonts(&cc.egui_ctx);
        Ok(Box::new(DemoApp::default()))
    }),
)
```

## 为什么推荐内置字体

- Windows、macOS、Linux 显示效果一致。
- 不依赖用户系统是否安装中文字体。
- 不需要维护不同操作系统的字体路径。
- 打包后离线也能正常显示中文。

代价是二进制体积会增加。当前使用的 `wqy-microhei.ttc` 约 5MB，比完整 Noto CJK 字体更小。

## 系统字体方案

开发阶段也可以读取系统字体，但不适合跨平台分发。

Ubuntu 常见字体安装命令：

```bash
sudo apt install fonts-noto-cjk fonts-wqy-microhei
```

Ubuntu 常见字体路径：

```text
/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc
/usr/share/fonts/truetype/wqy/wqy-microhei.ttc
/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc
/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf
```

macOS 常见中文字体：

```text
/System/Library/Fonts/PingFang.ttc
/System/Library/Fonts/STHeiti Light.ttc
```

Windows 常见中文字体：

```text
C:\Windows\Fonts\msyh.ttc
C:\Windows\Fonts\simsun.ttc
```

这些路径不能保证所有机器都存在，所以正式应用仍建议内置字体。

## 排查步骤

确认内置字体文件存在：

```bash
ls -lh assets/fonts/wqy-microhei.ttc
```

确认代码编译：

```bash
cargo check
```

如果中文仍然显示方块，重点检查：

- `configure_chinese_fonts(&cc.egui_ctx)` 是否在 UI 第一次渲染前调用。
- 字体是否同时加入了 `Proportional` 和 `Monospace`。
- `include_bytes!` 的相对路径是否从当前源码文件位置计算。
- 使用的字体文件是否真的包含中文 glyph。

## 注意事项

- `.ttc`、`.ttf`、`.otf` 都可以作为字体源，前提是 `egui` 版本支持对应格式解析。
- `include_bytes!("../assets/fonts/wqy-microhei.ttc")` 的路径是相对 `src/main.rs` 所在目录。
- 如果项目需要更完整的中日韩字体覆盖，可以改用 Noto Sans CJK，但文件体积更大。
- 如果字体许可证有分发限制，需要换成允许随应用分发的字体。

