//! 经纬度网格、无人机标记与点击导航。
use shared::{latlon_to_uv, uv_to_latlon, DroneState};
use std::collections::HashMap;

pub fn render<F>(
    ui: &mut egui::Ui,
    drones: &HashMap<String, DroneState>,
    selected: Option<&str>,
    mut on_click: F,
) where
    F: FnMut(String, f64, f64),
{
    let (resp, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click());
    let rect = resp.rect;
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(19, 28, 40));
    for i in 0..=10 {
        let x = rect.left() + rect.width() * i as f32 / 10.0;
        let y = rect.top() + rect.height() * i as f32 / 10.0;
        let stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(46, 60, 76));
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
    for d in drones.values() {
        let (u, v) = latlon_to_uv(d.lat, d.lon);
        let pos = egui::pos2(
            rect.left() + u * rect.width(),
            rect.top() + v * rect.height(),
        );
        let color = if Some(d.drone_id.as_str()) == selected {
            egui::Color32::YELLOW
        } else {
            egui::Color32::from_rgb(62, 213, 158)
        };
        if Some(d.drone_id.as_str()) == selected {
            painter.circle_stroke(pos, 12.0, egui::Stroke::new(1.5_f32, color));
        }
        painter.circle_filled(pos, 6.0, color);
        painter.text(
            pos + egui::vec2(10.0, -12.0),
            egui::Align2::LEFT_TOP,
            &d.drone_id,
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );
    }
    painter.text(
        rect.left_top() + egui::vec2(12.0, 12.0),
        egui::Align2::LEFT_TOP,
        "北 N  ·  29.89°",
        egui::FontId::proportional(13.0),
        egui::Color32::LIGHT_GRAY,
    );
    painter.text(
        rect.left_bottom() + egui::vec2(12.0, -12.0),
        egui::Align2::LEFT_BOTTOM,
        "121.52° E",
        egui::FontId::proportional(13.0),
        egui::Color32::LIGHT_GRAY,
    );
    painter.text(
        rect.right_bottom() + egui::vec2(-12.0, -12.0),
        egui::Align2::RIGHT_BOTTOM,
        "121.58° E / 29.85° N",
        egui::FontId::proportional(13.0),
        egui::Color32::LIGHT_GRAY,
    );
    if resp.clicked() {
        if let (Some(pos), Some(id)) = (resp.interact_pointer_pos(), selected) {
            let (lat, lon) = uv_to_latlon(
                (pos.x - rect.left()) / rect.width(),
                (pos.y - rect.top()) / rect.height(),
            );
            on_click(id.to_owned(), lat, lon);
        }
    }
}
