//! 成员5：GUI 入口。网络及视频模块保留成员6的替换接口。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod fonts;
mod map;
mod net;
mod video;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "无人机指挥站（宁波）——成员5界面")]
struct Args {
    #[arg(long, default_value = "127.0.0.1:9000")]
    server: String,
    /// 仅用示例状态预览界面；不连接服务器、不控制真实无人机。
    #[arg(long)]
    demo: bool,
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([900.0, 620.0]),
        ..Default::default()
    };
    // eframe 0.27 的创建闭包直接返回 Box<dyn App>，不包一层 Ok。
    eframe::run_native(
        "无人机指挥站（宁波）",
        options,
        Box::new(move |cc| Box::new(app::App::new(cc, args.server, args.demo))),
    )
}
