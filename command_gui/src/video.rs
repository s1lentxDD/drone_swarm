//! 成员6待实现：保持 render 签名，后续直接替换此文件。
use shared::DroneState;
use std::collections::HashMap;

pub fn render(ui: &mut egui::Ui, _drones: &HashMap<String, DroneState>, _selected: Option<&str>) {
    ui.heading("视频流");
    ui.label("视频模块待成员6接入");
}
