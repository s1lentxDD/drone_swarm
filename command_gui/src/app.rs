//! 成员5：应用状态、列表、控制面板、日志与地图。
use shared::{Command, DroneState, GuiMsg, ServerMsg};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct App {
    pub drones: HashMap<String, DroneState>,
    pub selected: Option<String>,
    pub logs: Vec<String>,
    pub rx: mpsc::Receiver<String>,
    pub tx: mpsc::Sender<String>,
    demo: bool,
    server: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, server: String, demo: bool) -> Self {
        crate::fonts::install(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (tx_to_net, rx_from_gui) = mpsc::channel(64);
        let (tx_to_gui, rx_from_net) = mpsc::channel(64);
        if !demo {
            crate::net::spawn(server.clone(), rx_from_gui, tx_to_gui);
        }
        let mut app = Self {
            drones: HashMap::new(),
            selected: None,
            logs: vec![],
            rx: rx_from_net,
            tx: tx_to_net,
            demo,
            server,
        };
        if demo {
            app.apply_message(ServerMsg::Snapshot {
                drones: vec![
                    DroneState {
                        drone_id: "d1".into(),
                        lat: 29.872,
                        lon: 121.542,
                        alt: 10.0,
                        battery: 86.5,
                        status: "flying".into(),
                    },
                    DroneState {
                        drone_id: "d2".into(),
                        lat: 29.865,
                        lon: 121.561,
                        alt: 0.0,
                        battery: 100.0,
                        status: "idle".into(),
                    },
                ],
            });
            app.add_log("界面演示：d1、d2 为静态示例数据，不连接服务器。".into());
        }
        app
    }

    fn add_log(&mut self, line: String) {
        self.logs.push(line);
        if self.logs.len() > 200 {
            self.logs.remove(0);
        }
    }

    fn apply_message(&mut self, msg: ServerMsg) {
        match msg {
            ServerMsg::Snapshot { drones } => {
                self.drones = drones
                    .into_iter()
                    .map(|d| (d.drone_id.clone(), d))
                    .collect();
                if self
                    .selected
                    .as_ref()
                    .is_some_and(|id| !self.drones.contains_key(id))
                {
                    self.selected = None;
                }
            }
            ServerMsg::Log { text } => self.add_log(text),
        }
    }

    pub fn send_cmd(&mut self, drone_id: &str, command: Command) {
        let msg = GuiMsg::SendCommand {
            drone_id: drone_id.into(),
            command,
        };
        let line = match serde_json::to_string(&msg) {
            Ok(line) => line,
            Err(err) => {
                self.add_log(format!("指令编码失败：{err}"));
                return;
            }
        };
        if self.demo {
            self.add_log(format!("演示指令（未发送）：{line}"));
            return;
        }
        match self.tx.try_send(line) {
            Ok(()) => self.add_log(format!("{drone_id} 指令已交给网络队列，等待服务器处理。")),
            Err(err) => self.add_log(format!("指令未发送：{err}")),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(line) = self.rx.try_recv() {
            match serde_json::from_str::<ServerMsg>(&line) {
                Ok(msg) => self.apply_message(msg),
                Err(err) => self.add_log(format!("忽略无法解析的服务器消息：{err}")),
            }
        }
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading("无人机指挥站 · 宁波");
                ui.separator();
                if self.demo {
                    ui.colored_label(egui::Color32::YELLOW, "界面演示 · 示例数据 · 不发送指令");
                } else {
                    ui.label(format!("服务器配置：{}", self.server));
                    if self.tx.is_closed() {
                        ui.colored_label(egui::Color32::YELLOW, "网络通道未连接");
                    }
                }
            });
            ui.add_space(8.0);
        });
        egui::SidePanel::left("list")
            .default_width(230.0)
            .width_range(190.0..=320.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("无人机列表");
                ui.label(format!("当前 {} 架", self.drones.len()));
                ui.separator();
                if self.drones.is_empty() {
                    ui.label("暂无无人机状态，等待服务器数据。");
                }
                let mut ids: Vec<_> = self.drones.keys().cloned().collect();
                ids.sort();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for id in ids {
                        let d = &self.drones[&id];
                        let label = format!("{}  |  {:.1}%  |  {}", id, d.battery, d.status);
                        if ui
                            .selectable_label(self.selected.as_deref() == Some(id.as_str()), label)
                            .clicked()
                        {
                            self.selected = Some(id);
                        }
                    }
                });
            });
        egui::SidePanel::right("right")
            .default_width(280.0)
            .width_range(250.0..=400.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("指令");
                if let Some(id) = self.selected.clone() {
                    ui.label(format!("已选择：{id}"));
                    if let Some(d) = self.drones.get(&id) {
                        ui.label(format!("高度 {:.1} m  ·  电量 {:.1}%", d.alt, d.battery));
                        ui.label(format!("{:.5}° N, {:.5}° E", d.lat, d.lon));
                    }
                    ui.horizontal(|ui| {
                        if ui.button("起飞").clicked() {
                            self.send_cmd(&id, Command::Takeoff);
                        }
                        if ui.button("降落").clicked() {
                            self.send_cmd(&id, Command::Land);
                        }
                    });
                } else {
                    ui.label("请先在左侧选择一架无人机。");
                    ui.add_enabled(false, egui::Button::new("起飞"));
                    ui.add_enabled(false, egui::Button::new("降落"));
                }
                ui.separator();
                crate::video::render(ui, &self.drones, self.selected.as_deref());
                ui.separator();
                ui.heading("日志");
                egui::ScrollArea::vertical()
                    .id_source("logs")
                    .show(ui, |ui| {
                        for line in self.logs.iter().rev().take(20) {
                            ui.label(line);
                            ui.add_space(4.0);
                        }
                    });
            });
        let mut clicked = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("位置地图");
            ui.label("选择无人机后点击网格，生成前往该坐标的指令。绿点：未选中；黄点：已选中。");
            ui.add_space(6.0);
            crate::map::render(
                ui,
                &self.drones,
                self.selected.as_deref(),
                |id, lat, lon| {
                    clicked = Some((id, lat, lon));
                },
            );
        });
        if let Some((id, lat, lon)) = clicked {
            self.send_cmd(&id, Command::Goto { lat, lon });
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn harness() -> (App, mpsc::Receiver<String>) {
        let (tx, rx_out) = mpsc::channel(2);
        let (_tx, rx) = mpsc::channel(2);
        (
            App {
                drones: HashMap::new(),
                selected: None,
                logs: vec![],
                rx,
                tx,
                demo: false,
                server: String::new(),
            },
            rx_out,
        )
    }

    #[test]
    fn gui_command_matches_shared_wire_format() {
        let (mut app, mut out) = harness();
        app.send_cmd(
            "d2",
            Command::Goto {
                lat: 29.87,
                lon: 121.55,
            },
        );
        let msg: GuiMsg = serde_json::from_str(&out.try_recv().unwrap()).unwrap();
        assert!(
            matches!(msg, GuiMsg::SendCommand { drone_id, command: Command::Goto { lat, lon } } if drone_id == "d2" && lat == 29.87 && lon == 121.55)
        );
    }

    #[test]
    fn disconnected_drone_clears_selection() {
        let (mut app, _) = harness();
        app.selected = Some("d1".into());
        app.apply_message(ServerMsg::Snapshot { drones: vec![] });
        assert!(app.selected.is_none());
    }

    #[test]
    fn demo_does_not_send_and_closed_channel_reports_failure() {
        let (mut app, mut out) = harness();
        app.demo = true;
        app.send_cmd("d1", Command::Takeoff);
        assert!(out.try_recv().is_err());
        assert!(app.logs.last().unwrap().contains("未发送"));
        app.demo = false;
        drop(out);
        app.send_cmd("d1", Command::Land);
        assert!(app.logs.last().unwrap().starts_with("指令未发送"));
    }
}
