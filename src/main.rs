use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 360.0])
            .with_title("eframe UI Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "eframe UI Demo",
        options,
        Box::new(|cc| {
            configure_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(DemoApp::default()))
        }),
    )
}

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

#[derive(Default)]
struct DemoApp {
    name: String,
    count: i32,
    volume: f32,
    enabled: bool,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("简单 eframe UI Demo");
            ui.label("这是一个基于 egui/eframe 的桌面窗口示例。");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("名字:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    self.count -= 1;
                }

                ui.label(format!("计数: {}", self.count));

                if ui.button("+").clicked() {
                    self.count += 1;
                }
            });

            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).text("音量"));
            ui.checkbox(&mut self.enabled, "启用功能");

            ui.separator();

            let display_name = if self.name.trim().is_empty() {
                "Rust"
            } else {
                self.name.trim()
            };

            ui.label(format!(
                "Hello, {display_name}! count = {}, volume = {:.0}, enabled = {}",
                self.count, self.volume, self.enabled
            ));
        });
    }
}
